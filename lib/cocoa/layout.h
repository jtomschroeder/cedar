
#pragma once

#import <Cocoa/Cocoa.h>
#import <objc/runtime.h>
#include <vector>
#include <yoga/Yoga.h>

#include "layout.h"
#include "json/json.hpp"

@interface YGLayout : NSObject {
    NSView *_view;
    YGNode *_node;

    std::vector<NSView *> _children;
}

@property(nonatomic, weak, readonly) NSView *view;
@property(nonatomic, readonly) YGNode *node;

@end

@implementation YGLayout

@synthesize view = _view;
@synthesize node = _node;

- (instancetype)initWithView:(NSView *)view {
    if (self = [super init]) {
        _view = view;
        _node = YGNodeNew();

        // YGNodeSetContext(_node, (__bridge void *)view);

        // _isEnabled = NO;
        // _isIncludedInLayout = YES;

        YGNodeStyleSetFlexGrow(_node, 1.0);
    }

    return self;
}

- (void)dealloc {
    YGNodeFree(self.node);
    [super dealloc];
}

@end

//////////

static const void *kYGYogaAssociatedKey = &kYGYogaAssociatedKey;

@implementation NSView (YogaKit)

- (YGLayout *)yoga {
    YGLayout *yoga = objc_getAssociatedObject(self, kYGYogaAssociatedKey);
    if (!yoga) {
        yoga = [[YGLayout alloc] initWithView:self];
        objc_setAssociatedObject(self, kYGYogaAssociatedKey, yoga,
                                 OBJC_ASSOCIATION_RETAIN_NONATOMIC);
    }

    return yoga;
}

@end

@implementation YGLayout (Extra)

- (void)calculate {
    auto frame = self.view.frame;
    YGNodeCalculateLayout(self.node, frame.size.width, frame.size.height, YGDirectionInherit);

    // traverse subviews and 'update' frame
    dispatch_async(dispatch_get_main_queue(), ^{
      [self layout];
    });
}

- (void)insert:(NSView *)child {
    YGNodeInsertChild(self.node, child.yoga.node, YGNodeGetChildCount(self.node));
    _children.push_back(child);
}

- (void)layout {
    auto frame = NSMakeRect(YGNodeLayoutGetLeft(self.node), YGNodeLayoutGetTop(self.node),
                            YGNodeLayoutGetWidth(self.node), YGNodeLayoutGetHeight(self.node));

    [self.view setFrame:frame];

    for (auto child : _children) {
        [child.yoga layout];
    }
}

@end
