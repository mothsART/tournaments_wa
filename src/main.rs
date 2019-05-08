#[macro_use]
extern crate serde_derive;

use std::rc::Rc;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};
use fluent_bundle::{FluentBundle, FluentResource};

struct Model {
    console: ConsoleService,
    scene: Scene,
    rounds: Vec<Round>,
    winner: FightingPlayer
}

struct Round {
    inc: usize,
    fights: Vec<Fight>
}

impl Round {
    fn new() -> Round {
        Round {
            inc: 0,
            fights: Vec::new()
        }
    }
    
    fn push(&mut self, fight: Fight) {
        self.fights.push(fight);
    }
}

impl Iterator for Round {
    type Item = Fight;
    
    fn next(&mut self) -> Option<Fight> {
        self.inc += 1;
        Some(self.fights[self.inc - 1].clone())
    }
}

enum TournamentEvent {
    Create
}

type FightingPlayer = Option<Rc<Player>>;

#[derive(Debug, Clone)]
pub struct Fight {
    first_player: FightingPlayer,
    second_player: FightingPlayer
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    first_name: String,
    last_name: String
}

#[derive(Debug)]
pub enum Scene {
    Create,
    Tournament
}

impl Component for Model {
    type Message = TournamentEvent;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let player_one = Some(Rc::new(Player {
            first_name: String::from("Xenial"),
            last_name: String::from("Xerus")
        }));
        let player_two = Some(Rc::new(Player {
            first_name: String::from("Bionic"),
            last_name: String::from("Beaver")
        }));
        let player_three = Some(Rc::new(Player {
            first_name: String::from("Cosmic"),
            last_name: String::from("Cuttlefish")
        }));
        let player_four = Some(Rc::new(Player {
            first_name: String::from("Disco"),
            last_name: String::from("Dingo")
        }));
        let player_five = Some(Rc::new(Player {
            first_name: String::from("Artful"),
            last_name: String::from("Aardvark")
        }));
        let player_six = Some(Rc::new(Player {
            first_name: String::from("Zesty"),
            last_name: String::from("Zapus")
        }));
        let player_seven = Some(Rc::new(Player {
            first_name: String::from("Artful"),
            last_name: String::from("Aardvark")
        }));
        let player_height = Some(Rc::new(Player {
            first_name: String::from("Warty"),
            last_name: String::from("Warthog")
        }));
        let mut rounds = Vec::new();
        let mut round_one = Round::new();
        round_one.push(Fight {
            first_player: player_one.clone(),
            second_player:  player_two
        });
        round_one.push(Fight {
            first_player: player_three.clone(),
            second_player: player_four
        });
        round_one.push(Fight {
            first_player: player_five,
            second_player:  player_six.clone()
        });
        round_one.push(Fight {
            first_player: player_seven,
            second_player:  player_height.clone()
        });
        rounds.push(round_one);
        
        let mut round_two = Round::new();
        round_two.push(Fight {
            first_player: player_one.clone(),
            second_player:  player_three.clone()
        });
        round_two.push(Fight {
            first_player: player_six.clone(),
            second_player:  player_height.clone()
        });
        rounds.push(round_two);
        
        let mut round_three = Round::new();
        round_three.push(Fight {
            first_player: player_one.clone(),
            second_player:  player_six.clone()
        });
        rounds.push(round_three);
        Model {
            console: ConsoleService::new(),
            scene: Scene::Create,
            rounds: rounds,
            winner: player_six.clone()
        }
    }

    fn update(&mut self, event: Self::Message) -> ShouldRender {
        match event {
            TournamentEvent::Create => {
                self.console.log("create tournament !");
                self.scene = Scene::Tournament;
            }
        }
        true
    }
}

fn create_tournament() -> String {
    let ftl_string = String::from("create-tournament = Créer un tournoi");
    let res = FluentResource::try_new(ftl_string)
        .expect("Could not parse an FTL string.");
    let mut bundle = FluentBundle::new(&["fr-FR"]);
    bundle.add_resource(&res)
    .expect("Failed to add FTL resources to the bundle.");

    let (value, _errors) = bundle.format("create-tournament", None)
    .expect("Failed to format a message.");
    value
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        match self.scene {
            Scene::Create => html! {
                <div class="tournaments",>
                    <button onclick=|_| TournamentEvent::Create,>
                        { create_tournament() }
                    </button>
                </div>
            },
            Scene::Tournament => {
                let mut players = Vec::new();
                for fight in self.rounds[0].fights.iter() {
                    match &fight.first_player {
                        Some(player) => {
                            players.push(player);
                        },
                        None => {}
                    }
                    match &fight.second_player {
                        Some(player) => {
                            players.push(player);
                        },
                        None => {}
                    }
                }
                html! {
                    <div class="tournaments",>
                        <button onclick=|_| TournamentEvent::Create,>
                            { create_tournament() }
                        </button>
                        <div class="tournament",>
                            <div class="players",>
                                {
                                    for players.iter().map(|player| player.clone().view()) 
                                }
                            </div>
                            <div class="rounds",>
                                {
                                    for self.rounds.iter().map(Renderable::view)
                                }
                            </div>
                            <div class="winner",>
                                <div class="player",>
                                    { format!(
                                        "{} {}",
                                        self.winner.clone().unwrap().first_name,
                                        self.winner.clone().unwrap().last_name
                                      )
                                }
                                </div>
                            </div>
                        </div>
                    </div>
                }
            }
        }
    }
}

impl Renderable<Model> for Round {
    fn view(&self) -> Html<Model> {
        html! {
            <div class="round",>
                { for self.fights.iter().map(Renderable::view) }
            </div>
        }
    }
}

impl Renderable<Model> for Fight {
    fn view(&self) -> Html<Model> {
        if self.first_player.clone().is_none() && self.second_player.clone().is_none() {
            ()
        }
        html! {
            <div class="fight",>
                <div class="first opponent",>
                    { format!(
                        "{} {}",
                        self.first_player.clone().unwrap().first_name,
                        self.first_player.clone().unwrap().last_name
                      )
                    }
                </div>
                <div class="second opponent",>
                    { format!(
                        "{} {}",
                        self.second_player.clone().unwrap().first_name,
                        self.second_player.clone().unwrap().last_name
                      )
                    }
                </div>
            </div>
        }
    }
}

impl Renderable<Model> for Player {
    fn view(&self) -> Html<Model> {
        html! {
            <div class="player",>
                { format!(
                    "{} {}",
                    self.first_name,
                    self.last_name
                  )
                }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
