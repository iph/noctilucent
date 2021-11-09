#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    pub origin: Origin,
    pub name: String,
}

impl Reference {
    pub fn new(name: &str, origin: Origin) -> Reference {
        Reference {
            name: name.to_string(),
            origin,
        }
    }
    fn synthesize(&self) -> String {
        match &self.origin {
            Origin::Parameter => {
                format!("props.{}", self.name)
            }
            Origin::LogicalId => self.name.to_string(),
            Origin::Condition => self.name.to_string(),
            Origin::PseudoParameter(x) => match x {
                PseudoParameter::Partition => String::from("this.partition"),
                PseudoParameter::Region => String::from("this.region"),
                PseudoParameter::StackId => String::from("this.stackId"),
                PseudoParameter::StackName => String::from("this.stackName"),
                PseudoParameter::URLSuffix => String::from("this.urlSuffix"),
            },
        }
    }

    pub fn match_pseudo_parameter(val: &str) -> Option<PseudoParameter> {
        let pseudo = match val {
            "AWS::Region" => PseudoParameter::Region,
            "AWS::Partition" => PseudoParameter::Partition,
            "AWS::StackName" => PseudoParameter::StackName,
            "AWS::URLSuffix" => PseudoParameter::URLSuffix,
            "AWS::StackId" => PseudoParameter::StackId,
            &_ => return Option::None,
        };

        Option::Some(pseudo)
    }
}

// Origin for the ReferenceTable
#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    Parameter,
    LogicalId,
    Condition,
    PseudoParameter(PseudoParameter),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PseudoParameter {
    Partition,
    Region,
    StackId,
    StackName,
    URLSuffix,
}

#[test]
fn test_match_pseudo_parameters() {
    assert_eq!(
        Reference::match_pseudo_parameter("AWS::Region"),
        Option::Some(PseudoParameter::Region)
    );
    assert_eq!(
        Reference::match_pseudo_parameter("AWS::Partition"),
        Option::Some(PseudoParameter::Partition)
    );
    assert_eq!(
        Reference::match_pseudo_parameter("AWS::StackName"),
        Option::Some(PseudoParameter::StackName)
    );
    assert_eq!(
        Reference::match_pseudo_parameter("AWS::StackId"),
        Option::Some(PseudoParameter::StackId)
    );
    assert_eq!(
        Reference::match_pseudo_parameter("AWS::URLSuffix"),
        Option::Some(PseudoParameter::URLSuffix)
    );
    assert_eq!(
        Reference::match_pseudo_parameter("hello_world"),
        Option::None
    );
}
