
extern crate layout;

use layout::yoga::*;

fn main() {
    unsafe {
        let root = YGNodeNew();
        YGNodeStyleSetWidth(root, 500.);
        YGNodeStyleSetHeight(root, 120.);
        YGNodeStyleSetFlexDirection(root, YGFlexDirection::YGFlexDirectionRow);
        YGNodeStyleSetPadding(root, YGEdge::YGEdgeAll, 20.);

        let image = YGNodeNew();
        YGNodeStyleSetWidth(image, 80.);
        YGNodeStyleSetMargin(image, YGEdge::YGEdgeEnd, 20.);

        let text = YGNodeNew();
        YGNodeStyleSetHeight(text, 25.);
        YGNodeStyleSetAlignSelf(text, YGAlign::YGAlignCenter);
        YGNodeStyleSetFlexGrow(text, 1.);

        YGNodeInsertChild(root, image, 0);
        YGNodeInsertChild(root, text, 1);

        YGNodeCalculateLayout(root, 500., 400., YGDirection::YGDirectionLTR);

        println!("{}", YGNodeLayoutGetLeft(text));
        println!("{}", YGNodeLayoutGetTop(text));
        println!("{}", YGNodeLayoutGetRight(text));
        println!("{}", YGNodeLayoutGetBottom(text));
        println!("{}", YGNodeLayoutGetWidth(text));
        println!("{}", YGNodeLayoutGetHeight(text));

        // YGDirection YGNodeLayoutGetDirection(YGNodeRef node);

        // TODO: free nodes!
        // YGNodeFree();
    }
}
