use crate::{example::ExamplePage, example2};

use gloo_utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Url};
use web_sys::{DragEvent, HtmlImageElement};

use patternfly_yew::*;
use yew::prelude::*;

use chrono::Utc;

pub struct TableExample {
    model: SharedTableModel<ExampleEntry>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExampleEntry {
    pub entry: String,
}

impl TableRenderer for ExampleEntry {
    fn render(&self, column: ColumnIndex) -> Html {
        match column.index {
            0 => html! {{&self.entry}},
            _ => html! {},
        }
    }

    fn render_details(&self) -> Vec<Span> {
        vec![Span::max(html! {
            <>
                { "So many details for " }{ &self.entry }
            </>
        })]
    }
}

pub enum Msg {
	Dragged(DragEvent),
	Dropped(DragEvent),
    Append,
    Pop,
}

impl Component for TableExample {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let model = vec![
            ExampleEntry {
                entry: " ".into(),
            }
        ];

        Self {
            model: model.into(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::Append => self.model.push(ExampleEntry {
                entry: format!("Extra entry: {}", Utc::now()),
            }),
            Msg::Pop => {
                self.model.pop();
            },

            Msg::Dragged(event) => {
                event.prevent_default();
				return false;
            },
            Msg::Dropped(event) => {
                event.prevent_default();
                let data_transfer = event.data_transfer().unwrap().get_data("text").unwrap();
                self.model.push(ExampleEntry {
					entry: format!("Extra entry: {}", data_transfer),
				})
            }
		}

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		let header = html_nested! {
			<TableHeader>
				<TableColumn label=""/>
			</TableHeader>
		};
        html! {
            <>
				<Table<SharedTableModel<ExampleEntry>>
					mode={TableMode::CompactExpandable}
					header={header}
					entries={self.model.clone()}
				>
				</Table<SharedTableModel<ExampleEntry>>>
				<div class="drop-zone"
					ondragover={link.callback(|e| Msg::Dragged(e))}
					ondrop={link.callback(|e| Msg::Dropped(e))}>
                	<p>{ "drag queries here" }</p>
				</div>
				<div id="photos"></div>
            </>
        }
    }
}
