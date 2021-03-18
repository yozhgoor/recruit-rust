use crate::contributions::Contributions;
use crate::jobs::Jobs;
use candidate::{Availability, Candidate, ContractType};
use itertools::Itertools;
use yew::prelude::*;
use yewprint::{Card, Intent, Tag, Text, H1, H2};

pub struct Profile {
    props: ProfileProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ProfileProps {
    pub candidate: &'static Candidate,
    #[prop_or_default]
    pub highlighted_tech: Option<String>,
}

pub enum Msg {
    HighLight(String),
}

impl Component for Profile {
    type Message = Msg;
    type Properties = ProfileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Profile { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HighLight(value) => {
                if let Some(highlighted_tech) = self.props.highlighted_tech.as_mut() {
                    highlighted_tech.clear();
                    highlighted_tech.push_str(&value);
                } else {
                    self.props.highlighted_tech = Some(value);
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let tags = self
            .props
            .candidate
            .asked_techs
            .iter()
            .map(|x| {
                html! {
                    <Tag
                        class=classes!("tag")
                        interactive=true
                        intent={
                            match self.props.highlighted_tech.as_ref() {
                                Some(highlighted_tech) if highlighted_tech == x => Intent::Danger,
                                _ => Intent::Primary,
                            }
                        }
                        onclick=self.link.callback(move |_| Msg::HighLight(x.to_string()))
                    >
                        {x}
                    </Tag>
                }
            })
            .collect::<Html>();

        let pronouns = self.props.candidate.pronouns.iter().join("/");
        let urls = self
            .props
            .candidate
            .urls
            .iter()
            .map(|(x, y)| {
                html! {
                    <div class="url">
                        <a href={y}>{x}</a>
                        {" | "}
                    </div>
                }
            })
            .collect::<Html>();
        let contract = match self.props.candidate.contract_type {
            ContractType::Contractor => "Contractor",
            ContractType::Employee => "Employee",
            ContractType::Any => "Any",
        };
        let availability = match self.props.candidate.availability {
            Availability::FullTime => "Full time",
            Availability::PartTime => "Part time",
            Availability::NotAvailable => "Not available",
        };
        let jobs_list = self
            .props
            .candidate
            .jobs
            .iter()
            .map(|x| {
                html! {
                    <Jobs
                        job={x}
                        highlighted_tech=self.props.highlighted_tech.clone()
                        onclick=self.link.callback(|x| Msg::HighLight(x))
                    />
                }
            })
            .collect::<Html>();
        let contrib_list = self
            .props
            .candidate
            .contributions
            .iter()
            .map(|x| {
                html! {
                    <Contributions
                        contribution={x}
                        highlighted_tech=self.props.highlighted_tech.clone()
                        onclick=self.link.callback(|x| Msg::HighLight(x))
                    />
                }
            })
            .collect::<Html>();
        let personal_list = self
            .props
            .candidate
            .personal_projects
            .iter()
            .map(|x| {
                html! {
                    <Contributions
                        contribution={x}
                        highlighted_tech=self.props.highlighted_tech.clone()
                        onclick=self.link.callback(|x| Msg::HighLight(x))
                    />
                }
            })
            .collect::<Html>();

        html! {
            <Card
                class=classes!("candidate")
            >
                <div class="candidate-header-top">
                    <div class="candidate-tag">
                        {tags}
                    </div>
                    <Text class=classes!("candidate-bio")>{self.props.candidate.bio}</Text>
                </div>
                <div class="candidate-header-bottom">
                    <div class="candidate-name">
                        <H1>{self.props.candidate.name}</H1>
                        <Text class=classes!("candidate-pronouns")>{"("}{pronouns}{")"}</Text>
                    </div>
                    <div class="candidate-urls">
                        {urls}
                    </div>
                </div>
                <Text>{contract}</Text>
                <Text>{availability}</Text>
                <div class="candidate-jobs">
                    <H2>{"Jobs"}</H2>
                    {jobs_list}
                </div>
                <div class="candidate-contributions">
                    <H2>{"Contribution"}</H2>
                    {contrib_list}
                </div>
                <div class="candidate-personal">
                    <H2>{"Personal projects"}</H2>
                    {personal_list}
                </div>
            </Card>
        }
    }
}
