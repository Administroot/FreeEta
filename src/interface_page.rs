use iced::{
    widget::{column, horizontal_rule, image, mouse_area, row, vertical_space, Column},
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
        let page = row![
            self.default_pump_widget("static/png/Pump_1.png"),
            self.default_valve_1("static/png/Switch.png")
        ];
        page.into()
    }

    fn default_pump_widget<'a>(&self, pic_path: &str) -> Column<'a, MainMenuMessage> {
        let level = 1;
        column![
            vertical_space().height(Length::FillPortion(
                self.get_portion(level, SpacePosition::Upper)
            )),
            row![
                horizontal_rule(1),
                mouse_area(image(pic_path))
                    .on_press(MainMenuMessage::DragStart(level))
                    .on_release(MainMenuMessage::DragEnded(level)),
            ],
            vertical_space().height(Length::FillPortion(
                self.get_portion(level, SpacePosition::Lower)
            )),
        ]
    }

    fn default_valve_1<'a>(&self, pic_path: &str) -> Column<'a, MainMenuMessage> {
        let level = 2;
        column![
            vertical_space().height(Length::FillPortion(
                self.get_portion(level, SpacePosition::Upper)
            )),
            row![
                horizontal_rule(1),
                mouse_area(image(pic_path))
                    .on_press(MainMenuMessage::DragStart(level))
                    .on_release(MainMenuMessage::DragEnded(level)),
            ],
            vertical_space().height(Length::FillPortion(
                self.get_portion(level, SpacePosition::Lower)
            )),
        ]
    }

    /// Get Length::FillPortion(`value`)
    fn get_portion(&self, index: usize, position: SpacePosition) -> u16 {
        // Read from json
        let axis = &self.eta.nodes[index].pic.axis;
        let blocks = 102.;
        let axis_y = axis.y;

        if *self
            .is_dragging
            .get(index)
            .expect("Internal Error: Vector is_dragging is too short.")
        {
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
