import * as swcHelpers from "@swc/helpers";
// @filename: file.tsx
// @jsx: preserve
// @noLib: true
// @skipLibCheck: true
// @libFiles: react.d.ts,lib.d.ts
var React = require("react");
export default function Component(props) {
    return(// Error extra property
    /*#__PURE__*/ React.createElement(AnotherComponent, swcHelpers.extends({}, props, {
        Property1: true
    })));
};
function AnotherComponent(param) {
    var property1 = param.property1;
    return /*#__PURE__*/ React.createElement("span", null, property1);
}
