use nalgebra_glm::{Vec4,Vec2};

use crate::shapes::Shapes::{self, combine_triangles_level1, DrawableShape, Shape};

fn generate_arrow(
    p1: Vec2,
    p2: Vec2,
    head_percentage: f32,
    w1: f32,
    w2: f32,
    level: f32,
    color: Vec4,
) -> Shape {
    let dir = p2 - p1;
    let p1h = p1 + (1.0 - head_percentage) * dir;
    let quad = Shapes::Quad::new_from_two_points(&p1, &p1h, w1, &color, level).generate_shape(0);
    let head = Shapes::Triangle::new(p1h, p2, w2, Some(color), level).generate_shape(0);
    combine_triangles_level1([quad, head].to_vec())
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ArrowLocation {
    #[default]
    Center,
    Left,
    Right,
    Top,
    Bottom,
    TL,
    TR,
    BL,
    BR,
}
#[derive(typed_builder::TypedBuilder)]
pub struct XYCoordinates {
    // #[builder(setter(into))]
    x_range: (f32, f32),
    y_range: (f32, f32),
    density: (u32, u32),
    #[builder(default = 0.05)]
    line_width: f32,
    #[builder(default=None,setter(strip_option))]
    xy_subgrid: Option<(u32, u32)>,
    #[builder(default=Some(Vec4::new(0.4,0.8,1.0,1.0)),setter(strip_option))]
    main_color: Option<Vec4>,
    #[builder(default=Some(Vec4::new(0.2,0.4,0.50,0.5)),setter(strip_option))]
    seco_color: Option<Vec4>,

    /// (head%, quad width, triangle width)
    #[builder(default=Some((0.05,0.01,0.01)),setter(strip_option))]
    arrow_specification: Option<(f32, f32, f32)>,
    #[builder(default=Some(Vec4::new(1.0,1.0,1.0,0.5)),setter(strip_option))]
    arrow_color: Option<Vec4>,
    #[builder(default=Some(ArrowLocation::Center),setter(strip_option))]
    arrow_location: Option<ArrowLocation>,
}

impl XYCoordinates {
    fn generate_arrows(&self, level: f32) -> Shape {
        let arrow_specification = self.arrow_specification.unwrap();

        let mut grid = [
            Vec2::new(self.x_range.0, self.y_range.1),
            Vec2::new(0.5 * (self.x_range.0 + self.x_range.1), self.y_range.1),
            Vec2::new(self.x_range.1, self.y_range.1),
            //
            Vec2::new(self.x_range.0, 0.5 * (self.y_range.1 + self.y_range.0)),
            Vec2::new(
                0.5 * (self.x_range.0 + self.x_range.1),
                0.5 * (self.y_range.1 + self.y_range.0),
            ),
            Vec2::new(self.x_range.1, 0.5 * (self.y_range.1 + self.y_range.0)),
            //
            Vec2::new(self.x_range.0, self.y_range.0),
            Vec2::new(0.5 * (self.x_range.0 + self.x_range.1), self.y_range.0),
            Vec2::new(self.x_range.1, self.y_range.0),
        ];
        (
            arrow_specification.0,
            arrow_specification.1,
            arrow_specification.2,
        );
        let up_arrow = match self.arrow_location.unwrap() {
            // ArrowLocation::TL | ArrowLocation::Top | ArrowLocation::TR => None,

            //
            ArrowLocation::Left => Some(generate_arrow(
                grid[3],
                grid[0],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Center => Some(generate_arrow(
                grid[4],
                grid[1],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Right => Some(generate_arrow(
                grid[5],
                grid[2],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            //
            ArrowLocation::BL => Some(generate_arrow(
                grid[6],
                grid[0],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Bottom => Some(generate_arrow(
                grid[7],
                grid[1],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::BR => Some(generate_arrow(
                grid[8],
                grid[2],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            _ => None,
        };

        let down_arrow = match self.arrow_location.unwrap() {
            ArrowLocation::Left => Some(generate_arrow(
                grid[3],
                grid[6],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Center => Some(generate_arrow(
                grid[4],
                grid[7],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Right => Some(generate_arrow(
                grid[5],
                grid[8],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            //
            ArrowLocation::TL => Some(generate_arrow(
                grid[0],
                grid[6],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Top => Some(generate_arrow(
                grid[1],
                grid[7],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::TR => Some(generate_arrow(
                grid[2],
                grid[8],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),

            _ => None,
        };
        let left_arrow = match self.arrow_location.unwrap() {
            // ArrowLocation::TL | ArrowLocation::Top | ArrowLocation::TR => None,
            //
            ArrowLocation::TR => Some(generate_arrow(
                grid[2],
                grid[0],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Right => Some(generate_arrow(
                grid[5],
                grid[3],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::BR => Some(generate_arrow(
                grid[8],
                grid[6],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            //
            ArrowLocation::Top => Some(generate_arrow(
                grid[1],
                grid[0],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Center => Some(generate_arrow(
                grid[4],
                grid[3],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Bottom => Some(generate_arrow(
                grid[7],
                grid[6],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            _ => None,
        };

        let right_arrow = match self.arrow_location.unwrap() {
            // ArrowLocation::TL | ArrowLocation::Top | ArrowLocation::TR => None,
            //
            ArrowLocation::TL => Some(generate_arrow(
                grid[0],
                grid[2],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Left => Some(generate_arrow(
                grid[3],
                grid[5],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::BL => Some(generate_arrow(
                grid[6],
                grid[8],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            //
            ArrowLocation::Top => Some(generate_arrow(
                grid[1],
                grid[2],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Center => Some(generate_arrow(
                grid[4],
                grid[5],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            ArrowLocation::Bottom => Some(generate_arrow(
                grid[7],
                grid[8],
                arrow_specification.0,
                arrow_specification.1,
                arrow_specification.2,
                level,
                self.arrow_color.unwrap(),
            )),
            _ => None,
        };

        combine_triangles_level1(
            [
                left_arrow.unwrap_or_default(),
                right_arrow.unwrap_or_default(),
                up_arrow.unwrap_or_default(),
                down_arrow.unwrap_or_default(),
            ]
            .to_vec(),
        )
    }

    fn generate_grid(&self, level: f32) -> Shape {
        //
        let x_lines = if self.density.0 % 2 == 1 {
            self.density.0
        } else {
            self.density.0 + 1
        };
        //
        let y_lines = if self.density.1 % 2 == 1 {
            self.density.1
        } else {
            self.density.1 + 1
        };
        //

        let dx = (self.x_range.1 - self.x_range.0) / (self.density.0 as f32 - 1.0);
        let dy = (self.y_range.1 - self.y_range.0) / (self.density.1 as f32 - 1.0);

        // let mut lines: Vec<ShapeVertexType> = Vec::with_capacity((x_lines + y_lines) as usize);
        let number_of_lines = x_lines
            + y_lines
            + if let Some(subgrid) = self.xy_subgrid {
                subgrid.0 * (x_lines - 1) + subgrid.1 * (y_lines - 1)
            } else {
                0
            };

        let mut lines: Vec<Shape> = Vec::with_capacity(number_of_lines as usize);

        for i in 0..x_lines {
            let current_x_value = self.x_range.0 + i as f32 * dx;
            let current_line = Shapes::Quad::new_from_two_points(
                &Vec2::new(current_x_value, self.y_range.0),
                &Vec2::new(current_x_value, self.y_range.1),
                self.line_width,
                &self.main_color.unwrap(),
                level,
            );
            lines.push(current_line.generate_shape(0));
        }
        for i in 0..y_lines {
            let current_y_value = self.y_range.0 + i as f32 * dy;

            let current_line = Shapes::Quad::new_from_two_points(
                &Vec2::new(self.x_range.0, current_y_value),
                &Vec2::new(self.x_range.1, current_y_value),
                self.line_width,
                &self.main_color.unwrap(),
                level,
            );
            lines.push(current_line.generate_shape(0));
        }

        if let Some(subgrid) = self.xy_subgrid {
            //x subgrid
            let dx2 = (self.x_range.1 - self.x_range.0)
                / ((self.density.0 as f32 - 1.0) * (subgrid.0 as f32 + 1.0));
            let dy2 = (self.y_range.1 - self.y_range.0)
                / ((self.density.1 as f32 - 1.0) * (subgrid.1 as f32 + 1.0));

            for x in 0..x_lines - 1 {
                for s in 1..=subgrid.0 {
                    let current_x_value = self.x_range.0 + x as f32 * dx + s as f32 * dx2;
                    let current_line = Shapes::Quad::new_from_two_points(
                        &Vec2::new(current_x_value, self.y_range.0),
                        &Vec2::new(current_x_value, self.y_range.1),
                        self.line_width,
                        &self.seco_color.unwrap(),
                        level,
                    );
                    lines.push(current_line.generate_shape(0));
                }
            }

            for y in 0..y_lines - 1 {
                for s in 1..=subgrid.1 {
                    let current_y_value = self.x_range.0 + y as f32 * dy + s as f32 * dy2;
                    let current_line = Shapes::Quad::new_from_two_points(
                        &Vec2::new(self.x_range.0, current_y_value),
                        &Vec2::new(self.x_range.1, current_y_value),
                        self.line_width,
                        &self.seco_color.unwrap(),
                        level,
                    );
                    lines.push(current_line.generate_shape(0));
                }
            }
        }

        combine_triangles_level1(lines)
    }

    pub fn generate_shape(&self) -> Shape {
        let grid = self.generate_grid(0.001);
        let arrows = self.generate_arrows(0.002);
        combine_triangles_level1([grid, arrows].to_vec())
    }
}