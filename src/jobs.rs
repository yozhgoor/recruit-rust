use candidate::Job;
use yew::prelude::*;
use yewprint::{Tag, Text};

pub struct Jobs {
    props: Props,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    pub candidate_jobs: &'static Job,
}

impl Component for Jobs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Jobs { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let jobs_tags = self
            .props
            .candidate_jobs
            .tech
            .iter()
            .map(|x| {
                html! {
                    <Tag>
                        {x}
                    </Tag>
                }
            })
            .collect::<Html>();

        html! {
            <div>
                <Text>{self.props.candidate_jobs.company}</Text>
                <a href={self.props.candidate_jobs.website}>
                    {"Website"}
                </a>
                <Text>{self.props.candidate_jobs.description}</Text>
                <div>
                    {jobs_tags}
                </div>
                <Text>{self.props.candidate_jobs.period}</Text>
            </div>
        }
    }
}
