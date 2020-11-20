function invokeExternalFunction(idName, getFnName, setFnName) {
    var elem = document.getElementById(idName);

    elem.value = external.invoke(getFnName);

    elem.addEventListener("change", function(event) {
        external.invoke(setFnName + event.target.value);
    });
}

invokeExternalFunction("gain", "getGain", "setGain ");
invokeExternalFunction("output", "getOutput", "setOutput ");