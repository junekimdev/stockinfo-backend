use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Group<'a, 'input> {
    pub tag: &'a str,
    pub contexts: Vec<Context<'a>>,
    pub nodes: Vec<roxmltree::Node<'a, 'input>>,
}

impl<'a, 'input> Group<'a, 'input> {
    pub fn extract(doc: &'a roxmltree::Document<'input>, tag: &'a str) -> Group<'a, 'input> {
        let nodes = doc.root_element().children().filter(|child| {
            child.is_element()
                && child.tag_name().name().starts_with(tag)
                && !child.tag_name().name().ends_with("TextBlock")
        });

        let context_ids = nodes
            .clone()
            .map(|node| node.attribute("contextRef").unwrap());

        let contexts = Context::parse(doc, context_ids);

        Self {
            tag,
            contexts,
            nodes: nodes.collect::<Vec<roxmltree::Node>>(),
        }
    }

    pub fn to_vec_date_and_value(&'a self) -> Vec<(Period, String)> {
        let mut temp: HashMap<String, (Period, String)> = HashMap::new();
        for tag in self.transpose() {
            if tag.no_segment() {
                let k = tag.context.id.to_string();
                let v1 = tag.context.period.clone();
                let v2 = tag.text.unwrap().to_string();
                temp.entry(k).or_insert((v1, v2));
            }
        }
        let r: Vec<(Period, String)> = temp.values().cloned().collect();
        r
    }

    fn transpose(&'a self) -> Vec<ElementSingle<'a>> {
        self.nodes
            .iter()
            .map(|node| {
                let id = node.attribute("id").unwrap();
                let context_id = node.attribute("contextRef").unwrap();
                let text = node.text();
                let context = self
                    .contexts
                    .iter()
                    .find(|context| context.id == context_id)
                    .unwrap();

                ElementSingle {
                    id,
                    tag: self.tag,
                    context,
                    text,
                }
            })
            .collect::<Vec<ElementSingle>>()
    }
}

#[derive(Debug, Clone)]
pub struct ElementSingle<'a> {
    pub id: &'a str,
    pub tag: &'a str,
    pub context: &'a Context<'a>,
    pub text: Option<&'a str>,
}

impl<'a> ElementSingle<'a> {
    pub fn no_segment(&self) -> bool {
        self.context.entity.segment.is_none()
    }

    // pub fn is_segment(&self, member_tag: &str) -> bool {
    //     match &self.context.entity.segment {
    //         Some(seg) => {
    //             for mem in seg.members.iter() {
    //                 if mem.text == member_tag {
    //                     return true;
    //                 }
    //             }
    //             false
    //         }
    //         None => false,
    //     }
    // }

    // pub fn get_date(&self) -> Option<time::Date> {
    //     self.context.period.date
    // }

    // pub fn get_date_range(&self) -> (Option<time::Date>, Option<time::Date>) {
    //     (self.context.period.start_date, self.context.period.end_date)
    // }
}

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub id: &'a str,
    pub entity: Entity<'a>,
    pub period: Period,
}

impl<'a> Context<'a> {
    #[allow(unused)]
    fn parse(
        doc: &'a roxmltree::Document,
        ids: impl std::iter::Iterator<Item = &'a str> + Clone,
    ) -> Vec<Context<'a>> {
        doc.root_element()
            .children()
            .filter(|child| child.is_element() && child.has_tag_name("context"))
            .filter(|child| {
                let id = child.attribute("id").unwrap();
                ids.clone().any(|x| x == id)
            })
            .map(Context::from)
            .collect::<Vec<Context<'a>>>()
    }
}

impl<'a, 'input> From<roxmltree::Node<'a, 'input>> for Context<'a> {
    fn from(value: roxmltree::Node<'a, 'input>) -> Context<'a> {
        let id = value.attribute("id").unwrap();
        let mut entity: Option<Entity> = None;
        let mut period: Option<Period> = None;

        for child in value.children() {
            if !child.is_element() {
                continue;
            }

            let tag = child.tag_name().name();
            match tag {
                "entity" => entity = Some(Entity::from(child)),
                "period" => period = Some(Period::from(child)),
                _ => (),
            }
        }

        Self {
            id,
            entity: entity.unwrap(),
            period: period.unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity<'a> {
    pub segment: Option<Segment<'a>>,
}

impl<'a, 'input> From<roxmltree::Node<'a, 'input>> for Entity<'a> {
    fn from(value: roxmltree::Node<'a, 'input>) -> Entity<'a> {
        let mut segment: Option<Segment<'a>> = None;
        for child in value.children() {
            if !child.is_element() {
                continue;
            }

            let tag = child.tag_name().name();
            if tag != "segment" {
                continue;
            }

            segment = Some(Segment::from(child));
        }

        Self { segment }
    }
}

#[derive(Debug, Clone)]
pub struct Segment<'a> {
    pub members: Vec<SegmentMember<'a>>,
}

impl<'a, 'input> From<roxmltree::Node<'a, 'input>> for Segment<'a> {
    fn from(value: roxmltree::Node<'a, 'input>) -> Segment<'a> {
        let mut members = Vec::new();
        for child in value.children() {
            if !child.is_element() {
                continue;
            }

            members.push(SegmentMember::from(child));
        }
        Self { members }
    }
}

#[derive(Debug, Clone)]
pub struct SegmentMember<'a> {
    pub dimension: &'a str,
    pub text: &'a str,
}

impl<'a, 'input> From<roxmltree::Node<'a, 'input>> for SegmentMember<'a> {
    fn from(value: roxmltree::Node<'a, 'input>) -> SegmentMember<'a> {
        let dimension: &'a str = value.attribute("dimension").unwrap();
        let text: &'a str = value.text().unwrap();

        Self { dimension, text }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Period {
    pub date: Option<time::Date>,
    pub start_date: Option<time::Date>,
    pub end_date: Option<time::Date>,
}

impl Period {
    pub fn parse_date_from(d: &str) -> std::result::Result<time::Date, time::error::Parse> {
        let format = time::macros::format_description!("[year]-[month]-[day]");
        time::Date::parse(d, &format)
    }
}

impl From<roxmltree::Node<'_, '_>> for Period {
    fn from(value: roxmltree::Node) -> Self {
        let mut date: Option<time::Date> = None;
        let mut start_date: Option<time::Date> = None;
        let mut end_date: Option<time::Date> = None;

        for child in value.children() {
            if !child.is_element() {
                continue;
            }

            let tag = child.tag_name().name();
            let val = child.text().unwrap();
            match tag {
                "instant" => date = Some(Self::parse_date_from(val).unwrap()),
                "startDate" => start_date = Some(Self::parse_date_from(val).unwrap()),
                "endDate" => end_date = Some(Self::parse_date_from(val).unwrap()),
                _ => (),
            }
        }

        Self {
            date,
            start_date,
            end_date,
        }
    }
}
