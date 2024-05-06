use std::error::Error;
use std::iter::zip;

use regex::Regex;
use scraper::{Element, ElementRef, Html, Selector};

use crate::{LazyLock, Move};
use crate::character::CharacterId;

pub(crate) async fn load(character_id: &CharacterId) -> Result<Vec<Move>, Box<dyn Error>> {
    let html = request_data_page(character_id).await?;
    let move_identifiers = select_move_identifiers(&html);
    let move_blocks = select_move_blocks(&html);
    let zip = zip(move_identifiers, move_blocks);
    let moves: Vec<Move> = zip.filter_map(|(identifier, block)| parse_move(identifier, block)).collect();
    Ok(moves)
}

static MOVE_IDENTIFIER_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("div > div > section.section-collapsible > h5 > span").unwrap());
fn select_move_identifiers(html: &Html) -> Vec<ElementRef> {
    html.select(&MOVE_IDENTIFIER_SELECTOR)
        .filter(|id| !id.is_empty())
        .collect::<Vec<ElementRef>>()
}

static MOVE_BLOCK_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("div > div > section.section-collapsible > h5 + table.wikitable").unwrap());
fn select_move_blocks(html: &Html) -> Vec<ElementRef> {
    html.select(&MOVE_BLOCK_SELECTOR)
        .filter(|id| !id.is_empty())
        .collect::<Vec<ElementRef>>()
}

async fn request_data_page(character_id: &CharacterId) -> Result<Html, Box<dyn Error>> {
    let text = reqwest::get(character_id.frame_data_url()).await?.text().await?;
    Ok(Html::parse_document(&text))
}

static TABLE_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("tbody").unwrap());
static INPUT_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("tr > th > div > p > span").unwrap());
static NAME_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("tr > th > div > div").unwrap());
static HITBOX_IMAGE_ELEMENT_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("tr > th > a").unwrap());
static HITBOX_IMAGE_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(/images/thumb\S+) 2x").unwrap());
static DATA_ROW_SELECTOR: LazyLock<Selector> = LazyLock::new(|| Selector::parse("tr > td").unwrap());
const DEFAULT_IMAGE: &str = "https://wiki.supercombo.gg/images/thumb/4/42/SF6_Logo.png/300px-SF6_Logo.png";
fn parse_move(identifier: ElementRef, block: ElementRef) -> Option<Move> {
    let identifier = identifier.inner_html();
    let input = block.select(&INPUT_SELECTOR)
        .next()
        .map(|e| e.inner_html())?;
    let name = block.select(&NAME_SELECTOR)
        .next()
        .map(|e| e.inner_html())?;
    // need to initialize this as its own variable first since 'e' is consumed
    let mut select = block.select(&HITBOX_IMAGE_ELEMENT_SELECTOR).map(|e| e.html());
    let hitbox_image_url = {
        let image = select.next().and_then(hitbox_image_matcher);
        let hitbox = select.next().and_then(hitbox_image_matcher);
        hitbox.or(image).unwrap_or_else(|| DEFAULT_IMAGE.to_string())
    };
    let mut data = block.select(&DATA_ROW_SELECTOR)
        .map(|e| get_lowest_child(e))
        .map(|e| e.inner_html())
        .collect::<Vec<String>>()
        .into_iter();
    let damage = data.next().unwrap_or_else(|| String::from("-"));
    let chip_damage = data.next().unwrap_or_else(|| String::from("-"));
    let damage_scaling = data.next().unwrap_or_else(|| String::from("-"));
    let guard = data.next().unwrap_or_else(|| String::from("-"));
    let cancel = data.next().unwrap_or_else(|| String::from("-"));
    let hitconfirm_window = data.next().unwrap_or_else(|| String::from("-"));
    let startup = data.next().unwrap_or_else(|| String::from("-"));
    let active = data.next().unwrap_or_else(|| String::from("-"));
    let recovery = data.next().unwrap_or_else(|| String::from("-"));
    let total = data.next().unwrap_or_else(|| String::from("-"));
    let hitstun = data.next().unwrap_or_else(|| String::from("-"));
    let blockstun = data.next().unwrap_or_else(|| String::from("-"));
    let drive_damage_block = data.next().unwrap_or_else(|| String::from("-"));
    let drive_damage_hit = data.next().unwrap_or_else(|| String::from("-"));
    let drive_gain = data.next().unwrap_or_else(|| String::from("-"));
    let super_gain_hit = data.next().unwrap_or_else(|| String::from("-"));
    let super_gain_block = data.next().unwrap_or_else(|| String::from("-"));
    let projectile_speed = data.next().unwrap_or_else(|| String::from("-"));
    let invuln = data.next().unwrap_or_else(|| String::from("-"));
    let armor = data.next().unwrap_or_else(|| String::from("-"));
    let airborne = data.next().unwrap_or_else(|| String::from("-"));
    let juggle_start = data.next().unwrap_or_else(|| String::from("-"));
    let juggle_increase = data.next().unwrap_or_else(|| String::from("-"));
    let juggle_limit = data.next().unwrap_or_else(|| String::from("-"));
    let perfect_parry_advantage = data.next().unwrap_or_else(|| String::from("-"));
    let after_dr_hit = data.next().unwrap_or_else(|| String::from("-"));
    let after_dr_block = data.next().unwrap_or_else(|| String::from("-"));
    let dr_cancel_hit = data.next().unwrap_or_else(|| String::from("-"));
    let dr_cancel_block = data.next().unwrap_or_else(|| String::from("-"));
    let punish_advantage = data.next().unwrap_or_else(|| String::from("-"));
    let hit_advantage = data.nth(10).unwrap_or_else(|| String::from("-"));
    let block_advantage = data.next().unwrap_or_else(|| String::from("-"));
    let notes = data.next().unwrap_or_else(|| String::from("-"));
    
    let move_constructed = Move {
        identifier,
        input,
        name,
        image_link: hitbox_image_url,
        damage,
        chip_damage,
        damage_scaling,
        guard,
        cancel,
        hitconfirm_window,
        startup,
        active,
        recovery,
        total,
        hitstun,
        blockstun,
        drive_damage_block,
        drive_damage_hit,
        drive_gain,
        super_gain_hit,
        super_gain_block,
        projectile_speed,
        invuln,
        armor,
        airborne,
        juggle_start,
        juggle_increase,
        juggle_limit,
        perfect_parry_advantage,
        after_dr_hit,
        after_dr_block,
        dr_cancel_hit,
        dr_cancel_block,
        punish_advantage,
        hit_advantage,
        block_advantage,
        notes,
    };
    Some(move_constructed)
}

fn get_lowest_child(parent: ElementRef) -> ElementRef {
    match parent.first_element_child() {
        None => parent,
        Some(child) => get_lowest_child(child)
    }
}

fn hitbox_image_matcher(element: String) -> Option<String> {
    HITBOX_IMAGE_URL_REGEX.captures(element.as_str())
        .and_then(|caps| caps.get(1))// skip first match: is whole match
        .map(|m| m.as_str().to_string())
        .map(|s| format!("https://wiki.supercombo.gg/{}", s))
}

