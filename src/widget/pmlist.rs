use tui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
};

use crate::{
    constant::LIST_H_MARGIN,
    pokemon::{DictType, Pokemon},
    AppState,
};

use super::dex::{PokemonDex, PokemonDexState};

fn flat_dex(pm: &Pokemon) -> PokemonDexState {
    let name = pm.name.get_name();
    let mut list = vec![PokemonDex {
        name: name.clone(),
        iv: pm.iv,
        pm_type: pm.get_type(),
        ability: pm.ability.clone(),
    }];

    match &pm.form {
        None => (),
        Some(form) => {
            for f in form {
                let name = format!("{} {}", name, f.form.join(" "));
                list.push(PokemonDex {
                    name,
                    iv: f.iv,
                    pm_type: f.get_type(),
                    ability: f.ability.clone(),
                });
            }
        }
    }

    let state = PokemonDexState {
        items: list,
        page: 1,
    };
    state
}

pub struct PokemonListStatus {
    pub state: ListState,
    pub items: Vec<Pokemon>,
    pub current: Pokemon,
    pub dex: PokemonDexState,

    items_clone: Vec<Pokemon>,
}

impl PokemonListStatus {
    pub fn new(mut items: Vec<Pokemon>) -> PokemonListStatus {
        // make sure items has def pokemon
        if items.len() == 0 {
            items.push(Pokemon::default());
        };

        // init position = 0
        let mut state = ListState::default();
        state.select(Some(0));

        let current = (&items).get(0).unwrap().clone();

        PokemonListStatus {
            state,
            dex: flat_dex(&current),
            current,
            items_clone: items.clone(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current(i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current(i);
    }

    pub fn set_list_filter(&mut self, filter: String) {
        if filter.eq("") {
            self.items = self.items_clone.clone();
        } else {
            self.items = self
                .items
                .iter()
                .filter(|item| {
                    item.get_list_name()
                        .to_lowercase()
                        .contains(filter.to_lowercase().as_str())
                })
                .map(|x| x.clone())
                .collect();
        }

        self.current(0);
    }

    fn current(&mut self, index: usize) {
        self.state.select(Some(index));
        self.current = match self.items.get(index) {
            Some(pm) => {
                let _pm = pm.clone();
                self.dex = flat_dex(&_pm);
                _pm
            }
            None => Pokemon::default(),
        };
    }
}

pub struct PokemonList;

impl Default for PokemonList {
    fn default() -> Self {
        PokemonList {}
    }
}

impl StatefulWidget for PokemonList {
    type State = AppState;

    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let AppState { pm, .. } = state;

        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .horizontal_margin(LIST_H_MARGIN)
            .split(area);

        let get_name = |item: &Pokemon| -> String {
            format!(
                "#{} {}",
                item.no.to_string().as_str(),
                item.name.get_name().as_str()
            )
        };
        let items: Vec<ListItem> = pm
            .items
            .iter()
            .filter(|item| {
                if state.query.eq("") {
                    return true;
                }

                get_name(item)
                    .to_lowercase()
                    .contains(state.query.to_lowercase().as_str())
            })
            .map(|item| ListItem::new(vec![Spans::from(item.get_list_name())]))
            .collect();

        List::new(items)
            .block(Block::default().borders(Borders::LEFT))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .render(layout[0], buf, &mut pm.state);
    }
}
