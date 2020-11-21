function setInitialValue(idName, getFnName) {
    var elem = document.getElementById(idName);
    elem.value = external.invoke(getFnName);
}

setInitialValue("gain", "getGain");
setInitialValue("output", "getOutput");

// Initialize the knob setting.
rotateKnob("gain", "gain-knob-image");
rotateKnob("output", "output-knob-image");