'use strict';

const addon = require('./build/Release/example.node');

addon.hello();
console.log(addon.add(1, 2));
