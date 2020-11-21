function setInitialValue(idName, getFnName) {
    var elem = document.getElementById(idName);
    elem.value = external.invoke(getFnName);
}

setInitialValue("gain", "getGain");
setInitialValue("output", "getOutput");