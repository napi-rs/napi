'use strict';

const addon = require('./build/Release/example.node');

addon.hello(undefined);
console.log(addon.add(1, 2));
