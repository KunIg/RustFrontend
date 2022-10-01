use crate::example::ExamplePage;

use patternfly_yew::*;
use yew::prelude::*;
use crate::queries::*;

pub struct Counter {
    value: i64,
}

#[derive(Clone, Debug)]
pub enum Msg {
    AddOne,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        log::info!("Clicks: {}", self.value);
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
