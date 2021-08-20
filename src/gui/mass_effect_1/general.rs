use std::cell::Ref;

use yew::{prelude::*, utils::NeqAssign};

use crate::save_data::{
    mass_effect_1::{
        data::{Data, Property as DataProperty},
        player::Player,
    },
    shared::plot::PlotTable,
    List,
};
use crate::{
    gui::{
        components::{Select, Table},
        mass_effect_1::property::Property,
        raw_ui::RawUi,
        RcUi,
    },
    save_data::mass_effect_1::data::StructType,
};

pub enum Msg {
    Difficulty(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub player: RcUi<Player>,
    pub plot: RcUi<PlotTable>,
}

impl Props {
    fn player(&self) -> Ref<'_, Player> {
        self.player.borrow()
    }

    fn plot(&self) -> Ref<'_, PlotTable> {
        self.plot.borrow()
    }
}

pub struct Me1General {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Me1General {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Me1General { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Difficulty(new_difficulty_idx) => {
                let player = self.props.player();

                // Worst thing ever >>
                // Find current game
                // Then find game options
                // Then find difficulty option
                let value = player
                    .objects
                    .iter()
                    .enumerate()
                    .find_map(|(i, object)| {
                        let object_name = player.get_name(object.object_name_id);
                        (object_name == "CurrentGame").then(|| player.get_data(i as i32 + 1))
                    })
                    .and_then(|current_game| {
                        let m_game_options =
                            self.find_property(&current_game.properties, "m_GameOptions")?.borrow();
                        match *m_game_options {
                            DataProperty::Struct {
                                struct_type: StructType::Properties(ref properties),
                                ..
                            } => Some(properties),
                            _ => None,
                        }
                        .and_then(|properties| {
                            self.find_property(properties, "m_nCombatDifficulty").and_then(|p| {
                                match *p.borrow() {
                                    DataProperty::Int { ref value, .. } => Some(RcUi::clone(value)),
                                    _ => None,
                                }
                            })
                        })
                    });

                // Then set new difficulty
                if let Some(mut value) = value {
                    *value.borrow_mut() = new_difficulty_idx as i32;
                }

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! { for self.try_view() }
    }
}

impl Me1General {
    fn try_view(&self) -> Option<Html> {
        let player = self.props.player();

        let current_game = player.objects.iter().enumerate().find_map(|(i, object)| {
            let object_name = player.get_name(object.object_name_id);
            (object_name == "CurrentGame").then(|| player.get_data(i as i32 + 1))
        })?;

        let m_player = {
            let object_id = self.find_object_id(&current_game.properties, "m_Player")?;
            player.get_data(object_id)
        };

        let m_squad = {
            let object_id = self.find_object_id(&m_player.properties, "m_Squad")?;
            player.get_data(object_id)
        };

        let m_inventory = {
            let object_id = self.find_object_id(&m_squad.properties, "m_Inventory")?;
            player.get_data(object_id)
        };

        Some(html! {
            <div class="flex divide-solid divide-x divide-default-border">
                <div class="flex-1 pr-1 flex flex-col gap-1">
                    { self.role_play(m_player) }
                    { self.gameplay(m_player, m_squad) }
                    { self.morality() }
                </div>
                <div class="flex-1 pl-1 flex flex-col gap-1">
                    { for self.general(&current_game.properties) }
                    { self.resources(m_inventory) }
                </div>
            </div>
        })
    }

    fn role_play(&self, m_player: &Data) -> Html {
        let name =
            self.find_property(&m_player.properties, "m_FirstName").map(|p| self.view_property(p));
        let gender =
            self.find_property(&m_player.properties, "m_Gender").map(|p| self.view_property(p));
        let origin = self
            .find_property(&m_player.properties, "m_BackgroundOrigin")
            .map(|p| self.view_property(p));
        let notoriety = self
            .find_property(&m_player.properties, "m_BackgroundNotoriety")
            .map(|p| self.view_property(p));

        html! {
            <Table title="Role-Play">
                { for name }
                { for gender }
                { for origin }
                { for notoriety }
            </Table>
        }
    }

    fn gameplay(&self, m_player: &Data, m_squad: &Data) -> Html {
        let class =
            self.find_property(&m_player.properties, "m_ClassBase").map(|p| self.view_property(p));
        let level =
            self.find_property(&m_player.properties, "m_XPLevel").map(|p| self.view_property(p));
        let curent_xp = self
            .find_property(&m_squad.properties, "m_nSquadExperience")
            .map(|p| self.view_property(p));

        html! {
            <Table title="Gameplay">
                { for class }
                { for level }
                { for curent_xp }
            </Table>
        }
    }

    fn morality(&self) -> Html {
        let plot = self.props.plot();
        html! {
            <Table title="Morality">
                { for plot.integers().get(47).map(|paragon| paragon.view("Paragon")) }
                { for plot.integers().get(46).map(|renegade| renegade.view("Renegade")) }
            </Table>
        }
    }

    fn general(&self, current_game: &List<RcUi<DataProperty>>) -> Option<Html> {
        let difficulty: &'static [&'static str] =
            &["Casual", "Normal", "Veteran", "Hardcore", "Insanity"];

        let m_game_options = self.find_property(current_game, "m_GameOptions")?.borrow();
        let difficulty = match *m_game_options {
            DataProperty::Struct {
                struct_type: StructType::Properties(ref properties), ..
            } => Some(properties),
            _ => None,
        }
        .and_then(|properties| {
            self.find_property(properties, "m_nCombatDifficulty").and_then(|p| match *p.borrow() {
                DataProperty::Int { ref value, .. } => Some(*value.borrow() as usize),
                _ => None,
            })
        })
        .map(|current_idx| {
            html! {
                <div class="flex items-center gap-1 cursor-default">
                    <Select
                        options={difficulty}
                        {current_idx}
                        onselect={self.link.callback(Msg::Difficulty)}
                    />
                    { "Difficulty" }
                </div>
            }
        });

        Some(html! {
            <Table title="General">
                { for difficulty }
            </Table>
        })
    }

    fn resources(&self, m_inventory: &Data) -> Html {
        let credits = self
            .find_property(&m_inventory.properties, "m_nResourceCredits")
            .map(|p| self.view_property(p));
        let medigel = self
            .find_property(&m_inventory.properties, "m_fResourceMedigel")
            .map(|p| self.view_property(p));
        let grenades = self
            .find_property(&m_inventory.properties, "m_nResourceGrenades")
            .map(|p| self.view_property(p));
        let omnigel = self
            .find_property(&m_inventory.properties, "m_fResourceSalvage")
            .map(|p| self.view_property(p));

        html! {
            <Table title="Resources">
                { for credits }
                { for medigel }
                { for grenades }
                { for omnigel }
            </Table>
        }
    }

    fn view_property(&self, property: &RcUi<DataProperty>) -> Html {
        let player = &self.props.player;
        html! {
            <Property
                player={RcUi::clone(player)}
                property={RcUi::clone(property)}
            />
        }
    }

    fn find_property<'a>(
        &self, properties: &'a List<RcUi<DataProperty>>, property_name: &str,
    ) -> Option<&'a RcUi<DataProperty>> {
        let player = self.props.player();
        properties.iter().find_map(|property| match *property.borrow() {
            DataProperty::Array { name_id, .. }
            | DataProperty::Bool { name_id, .. }
            | DataProperty::Byte { name_id, .. }
            | DataProperty::Float { name_id, .. }
            | DataProperty::Int { name_id, .. }
            | DataProperty::Name { name_id, .. }
            | DataProperty::Object { name_id, .. }
            | DataProperty::Str { name_id, .. }
            | DataProperty::StringRef { name_id, .. }
            | DataProperty::Struct { name_id, .. }
            | DataProperty::None { name_id, .. } => {
                (player.get_name(name_id) == property_name).then(|| property)
            }
        })
    }

    fn find_object_id(
        &self, properties: &List<RcUi<DataProperty>>, property_name: &str,
    ) -> Option<i32> {
        self.find_property(properties, property_name).and_then(|property| {
            match *property.borrow() {
                DataProperty::Object { object_id, .. } => Some(object_id),
                _ => None,
            }
        })
    }
}
