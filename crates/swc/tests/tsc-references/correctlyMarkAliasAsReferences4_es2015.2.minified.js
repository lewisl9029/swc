import * as swcHelpers from "@swc/helpers";
import * as cx from 'classnames';
import * as React from "react";
let buttonProps;
swcHelpers.extends({}, buttonProps, {
    className: cx('class1', {
        class2: !0
    })
});
