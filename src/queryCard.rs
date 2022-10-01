use patternfly_yew::*;
use yew::prelude::*;
use crate::queries::*;

pub struct QueryCard {
    table: Option<TableExample>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    AddOne,
}

impl Component for QueryCard {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { table: None }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.table = None,
        }
        //log::info!("Clicks: {}", self.value);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = html! {<>
            {""}
        </>};
        html! {
            <>
				<Gallery gutter=true>
					<Card
						selectable=true
						selected=true
						title={title}
					>
					<div></div>
					<Form>
						<FormGroup label="Search">
							<TextInput icon={TextInputIcon::Search}/>
						</FormGroup>
					</Form>
					<TableExample/>
					</Card>
				</Gallery>
            </>
        }
    }
}
