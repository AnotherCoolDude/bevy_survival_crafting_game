use bevy::prelude::*;

use crate::{
    item::WorldObject,
    prelude::{
        GameCamera, Inventory, ItemAndCount, ItemType, MousePosition, PlaceHolderGraphics,
        RESOLUTION,
    },
};

#[derive(Component)]
pub struct CraftingBox {
    //TODO grey out currently impossible recipes
    active: bool,
    //This is always a valid index, enforce this
    recipe_index: usize,
}

const CRAFTING_BOX_SIZE: f32 = 0.1;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CraftingBook {
            recipes: vec![
                CraftingRecipe {
                    needed: vec![
                        ItemAndCount {
                            item: ItemType::Twig,
                            count: 1,
                        },
                        ItemAndCount {
                            item: ItemType::Flint,
                            count: 1,
                        },
                    ],
                    produces: ItemType::Axe,
                },
                CraftingRecipe {
                    needed: vec![
                        ItemAndCount {
                            item: ItemType::Wood,
                            count: 1,
                        },
                        ItemAndCount {
                            item: ItemType::Grass,
                            count: 2,
                        },
                    ],
                    produces: ItemType::Fire,
                },
            ],
        })
        .add_startup_system(Self::spawn_crafting_ui)
        .add_system(Self::test_crafting);
    }
}

impl CraftingPlugin {
    fn test_crafting(
        mut inventory_query: Query<&mut Inventory>,
        box_query: Query<(&GlobalTransform, &CraftingBox)>,
        crafting_book: Res<CraftingBook>,
        mouse_pos: Res<MousePosition>,
        mouse: Res<Input<MouseButton>>,
    ) {
        for (transform, crafting_box) in box_query.iter() {
            assert!(transform.scale == Vec3::splat(1.0));
            let translation = transform.translation;
            //TODO use bevy aabb collide method
            if !(mouse_pos.0.x > translation.x - 0.5 * CRAFTING_BOX_SIZE
                && mouse_pos.0.x < translation.x + 0.5 * CRAFTING_BOX_SIZE
                && mouse_pos.0.y > translation.y - 0.5 * CRAFTING_BOX_SIZE
                && mouse_pos.0.y < translation.y + 0.5 * CRAFTING_BOX_SIZE)
            {
                continue;
            }
            //We are over a box
            let mut inventory = inventory_query.single_mut();
            if !mouse.just_pressed(MouseButton::Left) {
                continue;
            }
            //TODO just check if the button is active,
            // button should if can craft
            if !inventory.ingredients_available(&crafting_book.recipes[crafting_box.recipe_index]) {
                info!("neccessary ingredients for crafting not available");
                continue;
            }

            crafting_book.recipes[crafting_box.recipe_index]
                .needed
                .iter()
                .for_each(|item_and_count| {
                    if let Err(error) = inventory.remove(item_and_count) {
                        warn!("{:?}", error);
                    };
                });

            let product_item = crafting_book.recipes[crafting_box.recipe_index].produces;
            if let Some(overflow) = inventory.add(&ItemAndCount {
                item: product_item,
                count: 1,
            }) {
                warn!(
                    "couldnt add item to inventory: {}x{:?}",
                    overflow.0, product_item
                );
            };
        }
    }

    fn spawn_crafting_ui(
        mut commands: Commands,
        graphics: Res<PlaceHolderGraphics>,
        camera_query: Query<Entity, With<GameCamera>>,
        book: Res<CraftingBook>,
    ) {
        let camera_ent = camera_query.single();

        let spacing = 0.20;

        let starting_y = (book.recipes.len() as f32 / 2.0 + 0.5) * spacing;

        let mut sprite = TextureAtlasSprite::new(graphics.box_index);
        sprite.custom_size = Some(Vec2::splat(0.15));

        //could enumerate book
        let boxes: Vec<Entity> = book
            .recipes
            .iter()
            .enumerate()
            .map(|(i, recipe)| {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: sprite.clone(),
                        texture_atlas: graphics.texture_atlas.clone(),
                        transform: Transform::from_xyz(
                            -0.9 * RESOLUTION,
                            starting_y - spacing * i as f32,
                            -1.0,
                        ),
                        ..Default::default()
                    })
                    .insert(CraftingBox {
                        active: false,
                        recipe_index: i,
                    })
                    .insert(Name::new("CraftingBox"))
                    .with_children(|parent| {
                        let mut sprite = TextureAtlasSprite::new(
                            *graphics
                                .item_map
                                .get(&WorldObject::Item(recipe.produces))
                                .expect(&format!("No graphic for item {:?}", recipe.produces)),
                        );
                        sprite.custom_size = Some(Vec2::splat(CRAFTING_BOX_SIZE));
                        let graphic = parent
                            .spawn_bundle(SpriteSheetBundle {
                                sprite,
                                texture_atlas: graphics.texture_atlas.clone(),
                                ..Default::default()
                            })
                            .insert(Name::new("ItemGraphic"));
                    })
                    .id()
            })
            .collect();
        commands.entity(camera_ent).push_children(&boxes);
    }
}

pub struct CraftingBook {
    pub(crate) recipes: Vec<CraftingRecipe>,
}

#[derive(Clone)]
pub struct CraftingRecipe {
    pub(crate) needed: Vec<ItemAndCount>,
    pub(crate) produces: ItemType,
}

impl Inventory {
    pub fn ingredients_available(&mut self, recipe: &CraftingRecipe) -> bool {
        for ingredient in recipe.needed.clone() {
            if !self.can_remove(&ingredient) {
                return false;
            }
        }
        true
    }
}

fn can_craft(inventory: &Inventory, recipe: &CraftingRecipe) -> bool {
    recipe.needed.iter().any(|item_and_count| {
        inventory.items.iter().any(|item_slot| {
            item_slot.item == item_and_count.item && item_slot.count >= item_and_count.count
        })
    })
}
