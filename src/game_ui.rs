use core::fmt;

use crate::{
<<<<<<< HEAD
    item::ItemAndCount,
    prelude::{ActionsUI, HandUI, InventoryUI, RecipeUI},
=======
    item::{ItemAndCount, ItemType, WorldObject},
    prelude::{HandUI, InventoryUI, RecipeUI},
>>>>>>> 489ed5bf730c8c7f063dd707ee19ea32dff6bdc9
    GameState, HEIGHT, RESOLUTION,
};
use bevy::prelude::*;
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{
        bind, render, rsx,
        styles::{Edge, LayoutType, PositionType, Style as KayakStyle, StyleProp, Units},
        widget, Color, WidgetProps,
    },
    widgets::{App, Background, Clip},
};

pub struct GameUIPlugin;

/// Core event trigger by clicking on items in the UI
#[derive(Debug)]
pub struct UIEvent(pub UIEventType);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIEventType {
    None,
    CraftEvent(WorldObject),
    ToolEvent(ItemAndCount),
    InventoryEvent(ItemAndCount),
    ActionEvent(Action),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ShowItems,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
<<<<<<< HEAD
            Action::ShowItems => write!(f, "Show items"),
=======
            UIEventType::None => ItemAndCount {
                item: ItemType::None,
                count: 0,
            },
            UIEventType::CraftEvent(_i) => ItemAndCount {
                item: ItemType::None,
                count: 1,
            },
            UIEventType::ToolEvent(i) | UIEventType::InventoryEvent(i) => i,
>>>>>>> 489ed5bf730c8c7f063dd707ee19ea32dff6bdc9
        }
    }
}

impl Default for UIEventType {
    fn default() -> Self {
        UIEventType::None
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub inventory_items: Vec<ItemAndCount>,
    pub crafting_items: Vec<WorldObject>,
    pub hand_item: Option<ItemAndCount>,
}

#[derive(Default, Debug, WidgetProps, Clone, PartialEq)]
pub struct UIProps {
    #[prop_field(Styles)]
    pub styles: Option<KayakStyle>,
}

/// Main root widget for our game UI
#[widget]
fn GameUI() {
    let row_style = KayakStyle {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        col_between: StyleProp::Value(Units::Pixels(10.)),
        ..Default::default()
    };

    let column_style = KayakStyle {
        //padding: StyleProp::Value(Edge::axis(Units::Pixels(10.), Units::Stretch(1.0))),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(10.)),
        ..Default::default()
    };

<<<<<<< HEAD
    let screen_width = HEIGHT * RESOLUTION;

    rsx! {
        <>
            <Window position={(0., 0.)} size={(100., 500.)} title={"Inventory".to_string()}>
                <InventoryUI styles={Some(column_style)} />
            </Window>
            <Window position={(screen_width / 2. - 200., HEIGHT - 100.)} size={(400., 100.)} title={"Recipes".to_string()}>
                <RecipeUI styles={Some(row_style)} />
            </Window>
            <Window position={(screen_width - 200., HEIGHT - 100.)} size={(200., 100.)} title={"Hand Slot".to_string()} >
                <HandUI styles={Some(row_style)} />
            </Window>
            <Window position={(0., HEIGHT - 100.)} size={(200., 100.)} title={"Actions".to_string()} >
                <ActionsUI styles={Some(row_style)} />
            </Window>
        </>
=======
    let clip_styles = KayakStyle {
        padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        width: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    let width = 100.;
    let inventory_pos = (HEIGHT * RESOLUTION / 2., HEIGHT - width);
    let recipe_pos = (0., HEIGHT / 2.0);
    let hand_pos = (HEIGHT * RESOLUTION - width, HEIGHT - width);

    let recipe_style = KayakStyle {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Pixels(recipe_pos.0)),
        top: StyleProp::Value(Units::Pixels(recipe_pos.1)),
        //XXX these sizes should not matter ...
        width: StyleProp::Value(Units::Pixels(width)),
        height: StyleProp::Value(Units::Pixels(width)),
        max_width: StyleProp::Value(Units::Pixels(width)),
        max_height: StyleProp::Value(Units::Pixels(width)),
        background_color: StyleProp::Value(Color::new(0.7, 0.4, 0.4, 0.0)),
        ..Default::default()
    };

    let inventory_style = KayakStyle {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Pixels(inventory_pos.0)),
        top: StyleProp::Value(Units::Pixels(inventory_pos.1)),
        width: StyleProp::Value(Units::Pixels(width)),
        height: StyleProp::Value(Units::Pixels(width)),
        max_width: StyleProp::Value(Units::Pixels(width)),
        max_height: StyleProp::Value(Units::Pixels(width)),
        background_color: StyleProp::Value(Color::new(0.4, 0.7, 0.4, 0.0)),
        ..Default::default()
    };

    let hand_style = KayakStyle {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Pixels(hand_pos.0)),
        top: StyleProp::Value(Units::Pixels(hand_pos.1)),
        width: StyleProp::Value(Units::Pixels(width)),
        height: StyleProp::Value(Units::Pixels(width)),
        max_width: StyleProp::Value(Units::Pixels(width)),
        max_height: StyleProp::Value(Units::Pixels(width)),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.7, 0.0)),
        ..Default::default()
    };

    rsx! {
        <Clip styles= {Some(clip_styles)}>
            <Background styles={Some(inventory_style)}>
                <InventoryUI styles={Some(row_style)} />
            </Background>
            <Background styles={Some(recipe_style)}>
                <RecipeUI styles={Some(column_style)} />
            </Background>
            <Background styles={Some(hand_style)}>
                <HandUI styles={Some(row_style)} />
            </Background>
        </Clip>
>>>>>>> 489ed5bf730c8c7f063dd707ee19ea32dff6bdc9
    }
}

fn setup_game_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    commands.insert_resource(bind(UIItems::default()));

    let context = BevyContext::new(|context| {
        render! {
            <App>
                <GameUI />
            </App>
        }
    });

    commands.insert_resource(context);
}

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(setup_game_ui))
            .add_event::<UIEvent>();
    }
}
