import * as swcHelpers from "@swc/helpers";
// @allowUnreachableCode: true
// FunctionExpression with no return type annotation with multiple return statements with unrelated types
var f1 = function f1() {
    return "";
    return 3;
};
var f2 = function x() {
    return "";
    return 3;
};
var f3 = function() {
    return "";
    return 3;
};
// FunctionExpression with no return type annotation with return branch of number[] and other of string[]
var f4 = function f4() {
    if (true) {
        return [
            ""
        ];
    } else {
        return [
            1
        ];
    }
};
// Function implemetnation with non -void return type annotation with no return
function f5() {}
var m;
// Function signature with parameter initializer referencing in scope local variable
function f6() {
    var n = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : m;
    var m1 = 4;
}
// Function signature with initializer referencing other parameter to the right
function f7() {
    var n = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : m, m2 = arguments.length > 1 ? arguments[1] : void 0;
}
// FunctionExpression with non -void return type annotation with a throw, no return, and other code
// Should be error but isn't
undefined === function() {
    throw undefined;
    var x = 4;
};
var Base = function Base() {
    "use strict";
    swcHelpers.classCallCheck(this, Base);
};
var AnotherClass = function AnotherClass() {
    "use strict";
    swcHelpers.classCallCheck(this, AnotherClass);
};
var Derived1 = /*#__PURE__*/ function(Base) {
    "use strict";
    swcHelpers.inherits(Derived1, Base);
    var _super = swcHelpers.createSuper(Derived1);
    function Derived1() {
        swcHelpers.classCallCheck(this, Derived1);
        return _super.apply(this, arguments);
    }
    return Derived1;
}(Base);
var Derived2 = /*#__PURE__*/ function(Base) {
    "use strict";
    swcHelpers.inherits(Derived2, Base);
    var _super = swcHelpers.createSuper(Derived2);
    function Derived2() {
        swcHelpers.classCallCheck(this, Derived2);
        return _super.apply(this, arguments);
    }
    return Derived2;
}(Base);
function f8() {
    return new Derived1();
    return new Derived2();
}
var f9 = function f9() {
    return new Derived1();
    return new Derived2();
};
var f10 = function() {
    return new Derived1();
    return new Derived2();
};
function f11() {
    return new Base();
    return new AnotherClass();
}
var f12 = function f12() {
    return new Base();
    return new AnotherClass();
};
var f13 = function() {
    return new Base();
    return new AnotherClass();
};
