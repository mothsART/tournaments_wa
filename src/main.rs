#[macro_use]
extern crate serde_derive;

use std::rc::Rc;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::{ConsoleService};
use fluent_bundle::{FluentBundle, FluentResource};

struct Model {
    console: ConsoleService,
    scene: Scene,
    rounds: Vec<Round>
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
            first_name: String::from("jerem"),
            last_name: String::from("ferry")
        }));
        let player_two = Player {
            first_name: String::from("tom"),
            last_name: String::from("carotte")
        };
        let player_three = Some(Rc::new(Player {
            first_name: String::from("super"),
            last_name: String::from("star")
        }));
        let player_four = Player {
            first_name: String::from("mario"),
            last_name: String::from("bros")
        };
        let player_five = Player {
            first_name: String::from("princess"),
            last_name: String::from("peach")
        };
        let player_six = Player {
            first_name: String::from("luigi"),
            last_name: String::from("bros")
        };
        let mut rounds = Vec::new();
        let mut round_one = Round::new();
        round_one.push(Fight {
            first_player: player_one.clone(),
            second_player:  Some(Rc::new(player_two))
        });
        round_one.push(Fight {
            first_player: player_three.clone(),
            second_player:  Some(Rc::new(player_four))
        });
        round_one.push(Fight {
            first_player: Some(Rc::new(player_five)),
            second_player:  Some(Rc::new(player_six))
        });
        rounds.push(round_one);
        
        let mut round_two = Round::new();
        round_two.push(Fight {
            first_player: player_one.clone(),
            second_player:  player_three.clone()
        });
        rounds.push(round_two);
        Model {
            console: ConsoleService::new(),
            scene: Scene::Create,
            rounds: rounds
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
