// Use ES module import syntax to import functionality from the module
// that we have compiled.
//
// Note that the `default` import is an initialization function which
// will "boot" the module and make it ready to use. Currently browsers
// don't support natively imported WebAssembly as an ES module, but
// eventually the manual initialization won't be required!
import init, { convert_image, set_panic_hook } from './pkg/evbmfont_frontend.js';

async function run() {

await init();

set_panic_hook()

const imageInput = document.getElementById("image");
const widthInput = document.getElementById("width");
const heightInput = document.getElementById("height");
const firstChar = document.getElementById("firstChar");

const convertButton = document.getElementById("convert");
const copyButton = document.getElementById("copyButton");
const downloadButton = document.getElementById("downloadButton");
const fntOutput = document.getElementById("fntOutput");

function convert() {
	const selectedFile = imageInput.files[0]; // fetch the first uploaded file
	const width = widthInput.value;
	const height = heightInput.value;
	let first = firstChar.value;
	if (first == "") {
		first = " "
	}

	const reader = new FileReader();
	reader.onload = (e) => {
		let output = convert_image(
			new Uint8Array(e.target.result),
			selectedFile.name,
			width,
			height,
			first,
		);
		fntOutput.innerText = output;
		copyButton.hidden = false;
		downloadButton.hidden = false;
		downloadButton.href = window.URL.createObjectURL(new Blob([output]));
		downloadButton.download = stripExtension(selectedFile.name) + ".fnt";
	};
	reader.readAsArrayBuffer(selectedFile);
}

function stripExtension(inString) {
	let splitArray = inString.split(".");
	let outString = splitArray[0];
	for (let i = 1; i < splitArray.length - 1; i++) {
		outString += "." + splitArray[i];
	}
	return outString;
}

function copyOutput() {
	navigator.clipboard.writeText(fntOutput.innerText)
}

convertButton.addEventListener("click", convert);
copyButton.addEventListener("click", copyOutput);

}

run();
