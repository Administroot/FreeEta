use iced::{
    widget::{column, horizontal_rule, image, mouse_area, row, text, vertical_space, Column},
    Element, Length,
};

use crate::{
    freeeta_serial::{self, Axis},
    main_menu::{FreeEta, MainMenuMessage},
};

enum SpacePosition {
    Upper,
    Lower,
}

impl FreeEta {
    // Default interface page
    pub fn default_interface_page<'a>(&self) -> Element<'a, MainMenuMessage> {
        let page = self.default_pump_widget("static/png/Pump.png");
        page.into()
    }

    fn default_pump_widget<'a>(&self, pic_path: &str) -> Column<'a, MainMenuMessage> {
        column![
            vertical_space().height(Length::FillPortion(
                self.get_portion(1, SpacePosition::Upper)
            )),
            row![
                horizontal_rule(1),
                mouse_area(image(pic_path))
                    .on_press(MainMenuMessage::DragStart)
                    .on_release(MainMenuMessage::DragEnded),
                image("static/png/Switch.png"),
                text(format!("{:?}", self.window_size))
            ],
            vertical_space().height(Length::FillPortion(
                self.get_portion(1, SpacePosition::Lower)
            )),
        ]
    }

    /// Get Length::FillPortion(`value`)
    fn get_portion(&self, index: usize, position: SpacePosition) -> u16 {
        // TODO: 读取node，找到Y轴坐标。如果移动的话就改掉，没移动就直接返回
        let axis = read_node_axis(index).unwrap();
        let blocks = 102.;
        let axis_y = axis.y;

        if self.is_dragging {
            let window_height = self.window_size.height;
            let widget_y = self.mouse_point.y / window_height;
            let axis = Axis {
                x: axis.x,
                y: widget_y,
            };
            self.update_node_axis(index, axis).unwrap();
            match position {
                SpacePosition::Upper => (widget_y * blocks) as u16,
                SpacePosition::Lower => ((1. - widget_y) * blocks) as u16,
            }
        } else {
            match position {
                SpacePosition::Upper => (axis_y * blocks) as u16,
                SpacePosition::Lower => ((1. - axis_y) * blocks) as u16,
            }
        }
    }

    /// Serialize axis of a node (write to JSON file)
    fn update_node_axis(&self, index: usize, axis: Axis) -> Result<(), serde_json::Error> {
        let mut old_eta = self.eta.clone();
        match old_eta.nodes.get_mut(index) {
            Some(node) => {
                node.pic.axis = axis;
            }
            None => panic!("Node not found"),
        }

        // Write to JSON file
        freeeta_serial::update_eta_json("default_eta.json", old_eta)
    }
}

/// Get axis of a node (read from JSON file)
fn read_node_axis(index: usize) -> Result<Axis, Box<dyn std::error::Error>> {
    let eta_from_json =
        freeeta_serial::read_eta_json("default_eta.json").expect("JSON format error");
    match eta_from_json.nodes.get(index) {
        Some(node) => Ok(node.pic.axis.clone()),
        None => Err("Node not found".into()),
    }
}
