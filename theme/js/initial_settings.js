function setInitialValue(idName, getFnName) {
    var elem = document.getElementById(idName);
    elem.value = external.invoke(getFnName);
}

setInitialValue("gain", "getGain");
setInitialValue("output", "getOutput");
spinKnob("gain", "gain-knob-image");
spinKnob("output", "output-knob-image");