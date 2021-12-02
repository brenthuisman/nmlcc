#![allow(non_camel_case_types, non_snake_case, unused_variables)]
#![allow(clippy::many_single_char_names, clippy::large_enum_variant)]
use roxmltree::Node;

use crate::xml::XML;

#[derive(Debug, Clone)]
pub struct InstanceRequirement {
  pub name: String,
  pub r#type: String,
}

impl XML for InstanceRequirement {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let r#type = node.attribute("type").map(|s| s.to_string()).unwrap();
    InstanceRequirement { name, r#type }
  }
}

#[derive(Debug, Clone)]
pub enum ComponentTypeBody {
  Property(Property),
  Parameter(Parameter),
  DerivedParameter(DerivedParameter),
  IndexParameter(IndexParameter),
  Constant(Constant),
  Child(Child),
  Children(Children),
  Fixed(Fixed),
  Link(Link),
  ComponentReference(ComponentReference),
  Attachments(Attachments),
  EventPort(EventPort),
  Exposure(Exposure),
  Requirement(Requirement),
  ComponentRequirement(ComponentRequirement),
  InstanceRequirement(InstanceRequirement),
  Path(Path),
  Text(Text),
  Dynamics(Dynamics),
  Structure(Structure),
  Simulation(Simulation),
}

#[derive(Debug, Clone)]
pub struct ComponentType {
  pub name: String,
  pub extends: Option<String>,
  pub description: Option<String>,
  pub body: Vec<ComponentTypeBody>
}

impl XML for ComponentType {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let extends = node.attribute("extends").map(|s| s.to_string());
    let description = node.attribute("description").map(|s| s.to_string());
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "Property" => body.push(ComponentTypeBody::Property(Property::from_node(&child))),
        "Parameter" => body.push(ComponentTypeBody::Parameter(Parameter::from_node(&child))),
        "DerivedParameter" => body.push(ComponentTypeBody::DerivedParameter(DerivedParameter::from_node(&child))),
        "IndexParameter" => body.push(ComponentTypeBody::IndexParameter(IndexParameter::from_node(&child))),
        "Constant" => body.push(ComponentTypeBody::Constant(Constant::from_node(&child))),
        "Child" => body.push(ComponentTypeBody::Child(Child::from_node(&child))),
        "Children" => body.push(ComponentTypeBody::Children(Children::from_node(&child))),
        "Fixed" => body.push(ComponentTypeBody::Fixed(Fixed::from_node(&child))),
        "Link" => body.push(ComponentTypeBody::Link(Link::from_node(&child))),
        "ComponentReference" => body.push(ComponentTypeBody::ComponentReference(ComponentReference::from_node(&child))),
        "Attachments" => body.push(ComponentTypeBody::Attachments(Attachments::from_node(&child))),
        "EventPort" => body.push(ComponentTypeBody::EventPort(EventPort::from_node(&child))),
        "Exposure" => body.push(ComponentTypeBody::Exposure(Exposure::from_node(&child))),
        "Requirement" => body.push(ComponentTypeBody::Requirement(Requirement::from_node(&child))),
        "ComponentRequirement" => body.push(ComponentTypeBody::ComponentRequirement(ComponentRequirement::from_node(&child))),
        "InstanceRequirement" => body.push(ComponentTypeBody::InstanceRequirement(InstanceRequirement::from_node(&child))),
        "Path" => body.push(ComponentTypeBody::Path(Path::from_node(&child))),
        "Text" => body.push(ComponentTypeBody::Text(Text::from_node(&child))),
        "Dynamics" => body.push(ComponentTypeBody::Dynamics(Dynamics::from_node(&child))),
        "Structure" => body.push(ComponentTypeBody::Structure(Structure::from_node(&child))),
        "Simulation" => body.push(ComponentTypeBody::Simulation(Simulation::from_node(&child))),
        t => panic!("Unexpected tag {} in body of ComponentType.", t)
      };
    }
    ComponentType { name, extends, description, body }
  }
}

#[derive(Debug, Clone)]
pub enum OnEventBody {
  StateAssignment(StateAssignment),
  EventOut(EventOut),
}

#[derive(Debug, Clone)]
pub struct OnEvent {
  pub port: String,
  pub body: Vec<OnEventBody>
}

impl XML for OnEvent {
  fn from_node(node: &Node) -> Self {
    let port = node.attribute("port").map(|s| s.to_string()).unwrap();
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "StateAssignment" => body.push(OnEventBody::StateAssignment(StateAssignment::from_node(&child))),
        "EventOut" => body.push(OnEventBody::EventOut(EventOut::from_node(&child))),
        t => panic!("Unexpected tag {} in body of OnEvent.", t)
      };
    }
    OnEvent { port, body }
  }
}

#[derive(Debug, Clone)]
pub struct ChildInstance {
  pub component: String,
}

impl XML for ChildInstance {
  fn from_node(node: &Node) -> Self {
    let component = node.attribute("component").map(|s| s.to_string()).unwrap();
    ChildInstance { component }
  }
}

#[derive(Debug, Clone)]
pub struct EventRecord {
  pub quantity: String,
  pub eventPort: String,
}

impl XML for EventRecord {
  fn from_node(node: &Node) -> Self {
    let quantity = node.attribute("quantity").map(|s| s.to_string()).unwrap();
    let eventPort = node.attribute("eventPort").map(|s| s.to_string()).unwrap();
    EventRecord { quantity, eventPort }
  }
}

#[derive(Debug, Clone)]
pub struct Component {
}

impl XML for Component {
  fn from_node(node: &Node) -> Self {
    Component {  }
  }
}

#[derive(Debug, Clone)]
pub enum LemsBody {
  Target(Target),
  Include(Include),
  Dimension(Dimension),
  Unit(Unit),
  Constant(Constant),
  ComponentType(ComponentType),
  Component(Component),
}

#[derive(Debug, Clone)]
pub struct Lems {
  pub description: Option<String>,
  pub body: Vec<LemsBody>
}

impl XML for Lems {
  fn from_node(node: &Node) -> Self {
    let description = node.attribute("description").map(|s| s.to_string());
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "Target" => body.push(LemsBody::Target(Target::from_node(&child))),
        "Include" => body.push(LemsBody::Include(Include::from_node(&child))),
        "Dimension" => body.push(LemsBody::Dimension(Dimension::from_node(&child))),
        "Unit" => body.push(LemsBody::Unit(Unit::from_node(&child))),
        "Constant" => body.push(LemsBody::Constant(Constant::from_node(&child))),
        "ComponentType" => body.push(LemsBody::ComponentType(ComponentType::from_node(&child))),
        "Component" => body.push(LemsBody::Component(Component::from_node(&child))),
        t => panic!("Unexpected tag {} in body of Lems.", t)
      };
    }
    Lems { description, body }
  }
}

#[derive(Debug, Clone)]
pub struct Text {
  pub name: String,
  pub description: Option<String>,
}

impl XML for Text {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Text { name, description }
  }
}

#[derive(Debug, Clone)]
pub struct TimeDerivative {
  pub variable: String,
  pub value: String,
}

impl XML for TimeDerivative {
  fn from_node(node: &Node) -> Self {
    let variable = node.attribute("variable").map(|s| s.to_string()).unwrap();
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    TimeDerivative { variable, value }
  }
}

#[derive(Debug, Clone)]
pub struct Transition {
  pub regime: String,
}

impl XML for Transition {
  fn from_node(node: &Node) -> Self {
    let regime = node.attribute("regime").map(|s| s.to_string()).unwrap();
    Transition { regime }
  }
}

#[derive(Debug, Clone)]
pub enum StructureBody {
  ChildInstance(ChildInstance),
  MultiInstantiate(MultiInstantiate),
  ForEach(ForEach),
  With(With),
  Tunnel(Tunnel),
  EventConnection(EventConnection),
}

#[derive(Debug, Clone)]
pub struct Structure {
  pub body: Vec<StructureBody>
}

impl XML for Structure {
  fn from_node(node: &Node) -> Self {
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "ChildInstance" => body.push(StructureBody::ChildInstance(ChildInstance::from_node(&child))),
        "MultiInstantiate" => body.push(StructureBody::MultiInstantiate(MultiInstantiate::from_node(&child))),
        "ForEach" => body.push(StructureBody::ForEach(ForEach::from_node(&child))),
        "With" => body.push(StructureBody::With(With::from_node(&child))),
        "Tunnel" => body.push(StructureBody::Tunnel(Tunnel::from_node(&child))),
        "EventConnection" => body.push(StructureBody::EventConnection(EventConnection::from_node(&child))),
        t => panic!("Unexpected tag {} in body of Structure.", t)
      };
    }
    Structure { body }
  }
}

#[derive(Debug, Clone)]
pub struct Include {
  pub file: String,
}

impl XML for Include {
  fn from_node(node: &Node) -> Self {
    let file = node.attribute("file").map(|s| s.to_string()).unwrap();
    Include { file }
  }
}

#[derive(Debug, Clone)]
pub struct Record {
  pub quantity: String,
  pub timeScale: Option<String>,
  pub scale: Option<String>,
  pub color: Option<String>,
}

impl XML for Record {
  fn from_node(node: &Node) -> Self {
    let quantity = node.attribute("quantity").map(|s| s.to_string()).unwrap();
    let timeScale = node.attribute("timeScale").map(|s| s.to_string());
    let scale = node.attribute("scale").map(|s| s.to_string());
    let color = node.attribute("color").map(|s| s.to_string());
    Record { quantity, timeScale, scale, color }
  }
}

#[derive(Debug, Clone)]
pub struct Unit {
  pub symbol: String,
  pub dimension: String,
  pub power: i64,
  pub scale: f64,
  pub offset: f64,
}

impl XML for Unit {
  fn from_node(node: &Node) -> Self {
    let symbol = node.attribute("symbol").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").map(|s| s.to_string()).unwrap();
    let power = node.attribute("power").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    let scale = node.attribute("scale").or(Some("1")).map(|s| s.parse::<f64>().unwrap()).unwrap();
    let offset = node.attribute("offset").or(Some("0")).map(|s| s.parse::<f64>().unwrap()).unwrap();
    Unit { symbol, dimension, power, scale, offset }
  }
}

#[derive(Debug, Clone)]
pub struct Link {
  pub name: String,
  pub r#type: String,
  pub description: Option<String>,
}

impl XML for Link {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let r#type = node.attribute("type").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Link { name, r#type, description }
  }
}

#[derive(Debug, Clone)]
pub struct MultiInstantiate {
  pub component: String,
  pub number: String,
}

impl XML for MultiInstantiate {
  fn from_node(node: &Node) -> Self {
    let component = node.attribute("component").map(|s| s.to_string()).unwrap();
    let number = node.attribute("number").map(|s| s.to_string()).unwrap();
    MultiInstantiate { component, number }
  }
}

#[derive(Debug, Clone)]
pub struct Constant {
  pub name: String,
  pub dimension: String,
  pub value: String,
  pub description: Option<String>,
}

impl XML for Constant {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Constant { name, dimension, value, description }
  }
}

#[derive(Debug, Clone)]
pub struct Target {
  pub component: String,
  pub reportFile: Option<String>,
  pub timesFile: Option<String>,
}

impl XML for Target {
  fn from_node(node: &Node) -> Self {
    let component = node.attribute("component").map(|s| s.to_string()).unwrap();
    let reportFile = node.attribute("reportFile").map(|s| s.to_string());
    let timesFile = node.attribute("timesFile").map(|s| s.to_string());
    Target { component, reportFile, timesFile }
  }
}

#[derive(Debug, Clone)]
pub struct NamedDimensionalType {
  pub name: String,
  pub dimension: String,
  pub description: Option<String>,
}

impl XML for NamedDimensionalType {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    NamedDimensionalType { name, dimension, description }
  }
}

#[derive(Debug, Clone)]
pub enum ForEachBody {
  MultiInstantiate(MultiInstantiate),
}

#[derive(Debug, Clone)]
pub struct ForEach {
  pub instances: String,
  pub r#as: String,
  pub body: Vec<ForEachBody>
}

impl XML for ForEach {
  fn from_node(node: &Node) -> Self {
    let instances = node.attribute("instances").map(|s| s.to_string()).unwrap();
    let r#as = node.attribute("as").map(|s| s.to_string()).unwrap();
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "MultiInstantiate" => body.push(ForEachBody::MultiInstantiate(MultiInstantiate::from_node(&child))),
        t => panic!("Unexpected tag {} in body of ForEach.", t)
      };
    }
    ForEach { instances, r#as, body }
  }
}

#[derive(Debug, Clone)]
pub struct ComponentReference {
  pub name: String,
  pub r#type: String,
  pub local: Option<String>,
}

impl XML for ComponentReference {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let r#type = node.attribute("type").map(|s| s.to_string()).unwrap();
    let local = node.attribute("local").map(|s| s.to_string());
    ComponentReference { name, r#type, local }
  }
}

#[derive(Debug, Clone)]
pub struct Child {
  pub name: String,
  pub r#type: String,
  pub description: Option<String>,
}

impl XML for Child {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let r#type = node.attribute("type").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Child { name, r#type, description }
  }
}

#[derive(Debug, Clone)]
pub enum SimulationBody {
  DataDisplay(DataDisplay),
  Record(Record),
  EventRecord(EventRecord),
  Run(Run),
  DataWriter(DataWriter),
  EventWriter(EventWriter),
}

#[derive(Debug, Clone)]
pub struct Simulation {
  pub body: Vec<SimulationBody>
}

impl XML for Simulation {
  fn from_node(node: &Node) -> Self {
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "DataDisplay" => body.push(SimulationBody::DataDisplay(DataDisplay::from_node(&child))),
        "Record" => body.push(SimulationBody::Record(Record::from_node(&child))),
        "EventRecord" => body.push(SimulationBody::EventRecord(EventRecord::from_node(&child))),
        "Run" => body.push(SimulationBody::Run(Run::from_node(&child))),
        "DataWriter" => body.push(SimulationBody::DataWriter(DataWriter::from_node(&child))),
        "EventWriter" => body.push(SimulationBody::EventWriter(EventWriter::from_node(&child))),
        t => panic!("Unexpected tag {} in body of Simulation.", t)
      };
    }
    Simulation { body }
  }
}

#[derive(Debug, Clone)]
pub struct EventOut {
  pub port: String,
}

impl XML for EventOut {
  fn from_node(node: &Node) -> Self {
    let port = node.attribute("port").map(|s| s.to_string()).unwrap();
    EventOut { port }
  }
}

#[derive(Debug, Clone)]
pub struct Exposure {
  pub name: String,
  pub dimension: String,
  pub description: Option<String>,
}

impl XML for Exposure {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Exposure { name, dimension, description }
  }
}

#[derive(Debug, Clone)]
pub struct With {
  pub instance: Option<String>,
  pub list: Option<String>,
  pub index: Option<String>,
  pub r#as: String,
}

impl XML for With {
  fn from_node(node: &Node) -> Self {
    let instance = node.attribute("instance").map(|s| s.to_string());
    let list = node.attribute("list").map(|s| s.to_string());
    let index = node.attribute("index").map(|s| s.to_string());
    let r#as = node.attribute("as").map(|s| s.to_string()).unwrap();
    With { instance, list, index, r#as }
  }
}

#[derive(Debug, Clone)]
pub enum TunnelBody {
  Assign(Assign),
}

#[derive(Debug, Clone)]
pub struct Tunnel {
  pub name: String,
  pub endA: String,
  pub endB: String,
  pub componentA: String,
  pub componentB: String,
  pub body: Vec<TunnelBody>
}

impl XML for Tunnel {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let endA = node.attribute("endA").map(|s| s.to_string()).unwrap();
    let endB = node.attribute("endB").map(|s| s.to_string()).unwrap();
    let componentA = node.attribute("componentA").map(|s| s.to_string()).unwrap();
    let componentB = node.attribute("componentB").map(|s| s.to_string()).unwrap();
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "Assign" => body.push(TunnelBody::Assign(Assign::from_node(&child))),
        t => panic!("Unexpected tag {} in body of Tunnel.", t)
      };
    }
    Tunnel { name, endA, endB, componentA, componentB, body }
  }
}

#[derive(Debug, Clone)]
pub struct EventPort {
  pub name: String,
  pub direction: String,
  pub description: Option<String>,
}

impl XML for EventPort {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let direction = node.attribute("direction").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    EventPort { name, direction, description }
  }
}

#[derive(Debug, Clone)]
pub struct Assign {
  pub property: String,
  pub value: String,
}

impl XML for Assign {
  fn from_node(node: &Node) -> Self {
    let property = node.attribute("property").map(|s| s.to_string()).unwrap();
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    Assign { property, value }
  }
}

#[derive(Debug, Clone)]
pub struct Path {
  pub name: String,
  pub description: Option<String>,
}

impl XML for Path {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Path { name, description }
  }
}

#[derive(Debug, Clone)]
pub struct Case {
  pub condition: Option<String>,
  pub value: String,
}

impl XML for Case {
  fn from_node(node: &Node) -> Self {
    let condition = node.attribute("condition").map(|s| s.to_string());
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    Case { condition, value }
  }
}

#[derive(Debug, Clone)]
pub enum OnConditionBody {
  StateAssignment(StateAssignment),
  EventOut(EventOut),
  Transition(Transition),
}

#[derive(Debug, Clone)]
pub struct OnCondition {
  pub test: String,
  pub body: Vec<OnConditionBody>
}

impl XML for OnCondition {
  fn from_node(node: &Node) -> Self {
    let test = node.attribute("test").map(|s| s.to_string()).unwrap();
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "StateAssignment" => body.push(OnConditionBody::StateAssignment(StateAssignment::from_node(&child))),
        "EventOut" => body.push(OnConditionBody::EventOut(EventOut::from_node(&child))),
        "Transition" => body.push(OnConditionBody::Transition(Transition::from_node(&child))),
        t => panic!("Unexpected tag {} in body of OnCondition.", t)
      };
    }
    OnCondition { test, body }
  }
}

#[derive(Debug, Clone)]
pub struct DerivedVariable {
  pub name: String,
  pub dimension: String,
  pub exposure: Option<String>,
  pub description: Option<String>,
  pub select: Option<String>,
  pub value: Option<String>,
  pub reduce: Option<String>,
  pub required: Option<String>,
}

impl XML for DerivedVariable {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let exposure = node.attribute("exposure").map(|s| s.to_string());
    let description = node.attribute("description").map(|s| s.to_string());
    let select = node.attribute("select").map(|s| s.to_string());
    let value = node.attribute("value").map(|s| s.to_string());
    let reduce = node.attribute("reduce").map(|s| s.to_string());
    let required = node.attribute("required").map(|s| s.to_string());
    DerivedVariable { name, dimension, exposure, description, select, value, reduce, required }
  }
}

#[derive(Debug, Clone)]
pub struct StateAssignment {
  pub variable: String,
  pub value: String,
}

impl XML for StateAssignment {
  fn from_node(node: &Node) -> Self {
    let variable = node.attribute("variable").map(|s| s.to_string()).unwrap();
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    StateAssignment { variable, value }
  }
}

#[derive(Debug, Clone)]
pub struct Property {
  pub name: String,
  pub dimension: String,
  pub description: Option<String>,
  pub defaultValue: Option<f64>,
}

impl XML for Property {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    let defaultValue = node.attribute("defaultValue").map(|s| s.parse::<f64>().unwrap());
    Property { name, dimension, description, defaultValue }
  }
}

#[derive(Debug, Clone)]
pub enum DynamicsBody {
  StateVariable(StateVariable),
  DerivedVariable(DerivedVariable),
  ConditionalDerivedVariable(ConditionalDerivedVariable),
  TimeDerivative(TimeDerivative),
  OnStart(OnStart),
  OnEvent(OnEvent),
  OnCondition(OnCondition),
  Regime(Regime),
  KineticScheme(KineticScheme),
}

#[derive(Debug, Clone)]
pub struct Dynamics {
  pub body: Vec<DynamicsBody>
}

impl XML for Dynamics {
  fn from_node(node: &Node) -> Self {
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "StateVariable" => body.push(DynamicsBody::StateVariable(StateVariable::from_node(&child))),
        "DerivedVariable" => body.push(DynamicsBody::DerivedVariable(DerivedVariable::from_node(&child))),
        "ConditionalDerivedVariable" => body.push(DynamicsBody::ConditionalDerivedVariable(ConditionalDerivedVariable::from_node(&child))),
        "TimeDerivative" => body.push(DynamicsBody::TimeDerivative(TimeDerivative::from_node(&child))),
        "OnStart" => body.push(DynamicsBody::OnStart(OnStart::from_node(&child))),
        "OnEvent" => body.push(DynamicsBody::OnEvent(OnEvent::from_node(&child))),
        "OnCondition" => body.push(DynamicsBody::OnCondition(OnCondition::from_node(&child))),
        "Regime" => body.push(DynamicsBody::Regime(Regime::from_node(&child))),
        "KineticScheme" => body.push(DynamicsBody::KineticScheme(KineticScheme::from_node(&child))),
        t => panic!("Unexpected tag {} in body of Dynamics.", t)
      };
    }
    Dynamics { body }
  }
}

#[derive(Debug, Clone)]
pub struct Attachments {
  pub name: String,
  pub r#type: String,
  pub description: Option<String>,
}

impl XML for Attachments {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let r#type = node.attribute("type").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Attachments { name, r#type, description }
  }
}

#[derive(Debug, Clone)]
pub struct Fixed {
  pub parameter: String,
  pub value: String,
  pub description: Option<String>,
}

impl XML for Fixed {
  fn from_node(node: &Node) -> Self {
    let parameter = node.attribute("parameter").map(|s| s.to_string()).unwrap();
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Fixed { parameter, value, description }
  }
}

#[derive(Debug, Clone)]
pub struct StateVariable {
  pub name: String,
  pub dimension: String,
  pub exposure: Option<String>,
  pub description: Option<String>,
}

impl XML for StateVariable {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let exposure = node.attribute("exposure").map(|s| s.to_string());
    let description = node.attribute("description").map(|s| s.to_string());
    StateVariable { name, dimension, exposure, description }
  }
}

#[derive(Debug, Clone)]
pub enum OnEntryBody {
  StateAssignment(StateAssignment),
}

#[derive(Debug, Clone)]
pub struct OnEntry {
  pub body: Vec<OnEntryBody>
}

impl XML for OnEntry {
  fn from_node(node: &Node) -> Self {
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "StateAssignment" => body.push(OnEntryBody::StateAssignment(StateAssignment::from_node(&child))),
        t => panic!("Unexpected tag {} in body of OnEntry.", t)
      };
    }
    OnEntry { body }
  }
}

#[derive(Debug, Clone)]
pub struct DerivedParameter {
  pub name: String,
  pub dimension: String,
  pub value: String,
  pub description: Option<String>,
}

impl XML for DerivedParameter {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let value = node.attribute("value").map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    DerivedParameter { name, dimension, value, description }
  }
}

#[derive(Debug, Clone)]
pub struct Requirement {
  pub name: String,
  pub dimension: String,
  pub description: Option<String>,
}

impl XML for Requirement {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Requirement { name, dimension, description }
  }
}

#[derive(Debug, Clone)]
pub struct Dimension {
  pub name: String,
  pub m: i64,
  pub l: i64,
  pub t: i64,
  pub i: i64,
  pub k: i64,
  pub n: i64,
}

impl XML for Dimension {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let m = node.attribute("m").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    let l = node.attribute("l").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    let t = node.attribute("t").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    let i = node.attribute("i").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    let k = node.attribute("k").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    let n = node.attribute("n").or(Some("0")).map(|s| s.parse::<i64>().unwrap()).unwrap();
    Dimension { name, m, l, t, i, k, n }
  }
}

#[derive(Debug, Clone)]
pub struct DataWriter {
  pub path: String,
  pub fileName: String,
}

impl XML for DataWriter {
  fn from_node(node: &Node) -> Self {
    let path = node.attribute("path").map(|s| s.to_string()).unwrap();
    let fileName = node.attribute("fileName").map(|s| s.to_string()).unwrap();
    DataWriter { path, fileName }
  }
}

#[derive(Debug, Clone)]
pub struct Children {
  pub name: String,
  pub r#type: Option<String>,
  pub min: Option<i64>,
  pub max: Option<i64>,
  pub description: Option<String>,
}

impl XML for Children {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let r#type = node.attribute("type").map(|s| s.to_string());
    let min = node.attribute("min").map(|s| s.parse::<i64>().unwrap());
    let max = node.attribute("max").map(|s| s.parse::<i64>().unwrap());
    let description = node.attribute("description").map(|s| s.to_string());
    Children { name, r#type, min, max, description }
  }
}

#[derive(Debug, Clone)]
pub struct EventWriter {
  pub path: String,
  pub fileName: String,
  pub format: String,
}

impl XML for EventWriter {
  fn from_node(node: &Node) -> Self {
    let path = node.attribute("path").map(|s| s.to_string()).unwrap();
    let fileName = node.attribute("fileName").map(|s| s.to_string()).unwrap();
    let format = node.attribute("format").map(|s| s.to_string()).unwrap();
    EventWriter { path, fileName, format }
  }
}

#[derive(Debug, Clone)]
pub struct KineticScheme {
  pub name: String,
  pub nodes: String,
  pub stateVariable: String,
  pub edges: String,
  pub edgeSource: String,
  pub edgeTarget: String,
  pub forwardRate: String,
  pub reverseRate: String,
}

impl XML for KineticScheme {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let nodes = node.attribute("nodes").map(|s| s.to_string()).unwrap();
    let stateVariable = node.attribute("stateVariable").map(|s| s.to_string()).unwrap();
    let edges = node.attribute("edges").map(|s| s.to_string()).unwrap();
    let edgeSource = node.attribute("edgeSource").map(|s| s.to_string()).unwrap();
    let edgeTarget = node.attribute("edgeTarget").map(|s| s.to_string()).unwrap();
    let forwardRate = node.attribute("forwardRate").map(|s| s.to_string()).unwrap();
    let reverseRate = node.attribute("reverseRate").map(|s| s.to_string()).unwrap();
    KineticScheme { name, nodes, stateVariable, edges, edgeSource, edgeTarget, forwardRate, reverseRate }
  }
}

#[derive(Debug, Clone)]
pub struct DataDisplay {
  pub title: String,
  pub dataRegion: String,
}

impl XML for DataDisplay {
  fn from_node(node: &Node) -> Self {
    let title = node.attribute("title").map(|s| s.to_string()).unwrap();
    let dataRegion = node.attribute("dataRegion").map(|s| s.to_string()).unwrap();
    DataDisplay { title, dataRegion }
  }
}

#[derive(Debug, Clone)]
pub enum ConditionalDerivedVariableBody {
  Case(Case),
}

#[derive(Debug, Clone)]
pub struct ConditionalDerivedVariable {
  pub name: String,
  pub dimension: String,
  pub exposure: Option<String>,
  pub body: Vec<ConditionalDerivedVariableBody>
}

impl XML for ConditionalDerivedVariable {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let exposure = node.attribute("exposure").map(|s| s.to_string());
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "Case" => body.push(ConditionalDerivedVariableBody::Case(Case::from_node(&child))),
        t => panic!("Unexpected tag {} in body of ConditionalDerivedVariable.", t)
      };
    }
    ConditionalDerivedVariable { name, dimension, exposure, body }
  }
}

#[derive(Debug, Clone)]
pub struct ComponentRequirement {
  pub name: String,
}

impl XML for ComponentRequirement {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    ComponentRequirement { name }
  }
}

#[derive(Debug, Clone)]
pub enum OnStartBody {
  StateAssignment(StateAssignment),
}

#[derive(Debug, Clone)]
pub struct OnStart {
  pub body: Vec<OnStartBody>
}

impl XML for OnStart {
  fn from_node(node: &Node) -> Self {
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "StateAssignment" => body.push(OnStartBody::StateAssignment(StateAssignment::from_node(&child))),
        t => panic!("Unexpected tag {} in body of OnStart.", t)
      };
    }
    OnStart { body }
  }
}

#[derive(Debug, Clone)]
pub struct Run {
  pub component: String,
  pub variable: String,
  pub increment: String,
  pub total: String,
}

impl XML for Run {
  fn from_node(node: &Node) -> Self {
    let component = node.attribute("component").map(|s| s.to_string()).unwrap();
    let variable = node.attribute("variable").map(|s| s.to_string()).unwrap();
    let increment = node.attribute("increment").map(|s| s.to_string()).unwrap();
    let total = node.attribute("total").map(|s| s.to_string()).unwrap();
    Run { component, variable, increment, total }
  }
}

#[derive(Debug, Clone)]
pub enum RegimeBody {
  TimeDerivative(TimeDerivative),
  OnEntry(OnEntry),
  OnCondition(OnCondition),
}

#[derive(Debug, Clone)]
pub struct Regime {
  pub name: String,
  pub initial: Option<String>,
  pub body: Vec<RegimeBody>
}

impl XML for Regime {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let initial = node.attribute("initial").map(|s| s.to_string());
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "TimeDerivative" => body.push(RegimeBody::TimeDerivative(TimeDerivative::from_node(&child))),
        "OnEntry" => body.push(RegimeBody::OnEntry(OnEntry::from_node(&child))),
        "OnCondition" => body.push(RegimeBody::OnCondition(OnCondition::from_node(&child))),
        t => panic!("Unexpected tag {} in body of Regime.", t)
      };
    }
    Regime { name, initial, body }
  }
}

#[derive(Debug, Clone)]
pub struct Parameter {
  pub name: String,
  pub dimension: String,
  pub description: Option<String>,
}

impl XML for Parameter {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    let dimension = node.attribute("dimension").or(Some("none")).map(|s| s.to_string()).unwrap();
    let description = node.attribute("description").map(|s| s.to_string());
    Parameter { name, dimension, description }
  }
}

#[derive(Debug, Clone)]
pub struct IndexParameter {
  pub name: String,
}

impl XML for IndexParameter {
  fn from_node(node: &Node) -> Self {
    let name = node.attribute("name").map(|s| s.to_string()).unwrap();
    IndexParameter { name }
  }
}

#[derive(Debug, Clone)]
pub enum EventConnectionBody {
  Assign(Assign),
}

#[derive(Debug, Clone)]
pub struct EventConnection {
  pub from: String,
  pub to: String,
  pub sourcePort: Option<String>,
  pub targetPort: Option<String>,
  pub receiver: Option<String>,
  pub receiverContainer: Option<String>,
  pub delay: Option<String>,
  pub body: Vec<EventConnectionBody>
}

impl XML for EventConnection {
  fn from_node(node: &Node) -> Self {
    let from = node.attribute("from").map(|s| s.to_string()).unwrap();
    let to = node.attribute("to").map(|s| s.to_string()).unwrap();
    let sourcePort = node.attribute("sourcePort").map(|s| s.to_string());
    let targetPort = node.attribute("targetPort").map(|s| s.to_string());
    let receiver = node.attribute("receiver").map(|s| s.to_string());
    let receiverContainer = node.attribute("receiverContainer").map(|s| s.to_string());
    let delay = node.attribute("delay").map(|s| s.to_string());
    let mut body = Vec::new();
    for child in node.children() {
      if child.is_comment() || child.is_text() { continue; }
      match child.tag_name().name() {
        "Assign" => body.push(EventConnectionBody::Assign(Assign::from_node(&child))),
        t => panic!("Unexpected tag {} in body of EventConnection.", t)
      };
    }
    EventConnection { from, to, sourcePort, targetPort, receiver, receiverContainer, delay, body }
  }
}