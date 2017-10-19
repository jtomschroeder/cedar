
#pragma once

#import <Cocoa/Cocoa.h>
#import <objc/runtime.h>
#include <yoga/Yoga.h>

#include "layout.h"
#include "json/json.hpp"

@interface YGLayout : NSObject {
    NSView *_view;
    YGNode *_node;
}

@property(nonatomic, weak, readonly) NSView *view;
@property(nonatomic, readonly) YGNode *node;

@end

@implementation YGLayout

@synthesize view = _view;
@synthesize node = _node;

// + (void)initialize
// {
//   globalConfig = YGConfigNew();
//   YGConfigSetExperimentalFeatureEnabled(globalConfig, YGExperimentalFeatureWebFlexBasis, true);
//   YGConfigSetPointScaleFactor(globalConfig, [UIScreen mainScreen].scale);
// }

- (instancetype)initWithView:(NSView *)view {
    if (self = [super init]) {
        _view = view;
        _node = YGNodeNew();

        // YGNodeSetContext(_node, (__bridge void *)view);
        // _isEnabled = NO;
        // _isIncludedInLayout = YES;
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