function bigEar(inputIdName, earImageIdName) {
    var inputElem = document.getElementById(inputIdName);
    var earElem = document.getElementById(earImageIdName);

    earElem.style.width = "" + (50 + 50 * parseInt(inputElem.value) / 100) + "%";
}