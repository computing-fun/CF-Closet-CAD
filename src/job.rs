use rust_decimal::{Decimal, MathematicalOps};

use crate::math::angle::Angle;

#[derive(Default, Debug, Clone)]
pub struct Space {
    pub walls: Vec<WallSegment>,
}

impl Space {
    pub fn points(&self) -> impl Iterator<Item = Vec<(Decimal, Decimal)>> + '_ {
        self.walls.iter().map(|wall| wall.points())
    }
}

#[derive(Default, Debug, Clone)]
pub struct WallSegment {
    pub start_x: Decimal,
    pub start_y: Decimal,
    pub nodes: Vec<WallNode>,
}

impl WallSegment {
    pub fn points(&self) -> Vec<(Decimal, Decimal)> {
        let mut cursor_x = self.start_x;
        let mut cursor_y = self.start_y;
        let mut result = vec![(cursor_x, cursor_y)];
        for node in self.nodes.iter() {
            (cursor_x, cursor_y) = node.start_to_end(cursor_x, cursor_y);
            result.push((cursor_x, cursor_y));
        }
        return result;
    }
}

#[cfg(test)]
mod tests_wall {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::math::angle::Angle;

    use super::{WallNode, WallSegment};

    #[test]
    fn draw_square() {
        let mut wall = WallSegment::default();
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(dec!(90)),
            length: dec!(10),
        });
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(Decimal::ZERO),
            length: dec!(10),
        });
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(dec!(-90)),
            length: dec!(10),
        });
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(dec!(180)),
            length: dec!(10),
        });

        let result = wall.points();

        assert_eq!(result.len(), 5);
        assert_eq!(result[0], (dec!(0), dec!(0)));
        assert_eq!(result[1], (dec!(0), dec!(10)));
        assert_eq!(result[2], (dec!(10), dec!(10)));
        assert_eq!(result[3], (dec!(10), dec!(0)));
        assert_eq!(result[4], (dec!(0), dec!(0)));
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WallNode {
    pub angle: Angle,
    pub length: Decimal,
}

impl WallNode {
    pub fn start_to_end_x(&self, start: Decimal) -> Decimal {
        (start + self.length * self.angle.to_radians().cos()).round()
    }

    pub fn start_to_end_y(&self, start: Decimal) -> Decimal {
        (start + self.length * self.angle.to_radians().sin()).round()
    }

    pub fn start_to_end(&self, start_x: Decimal, start_y: Decimal) -> (Decimal, Decimal) {
        (self.start_to_end_x(start_x), self.start_to_end_y(start_y))
    }
}
