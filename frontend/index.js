import * as fluence from "fluence";

window.onload = function () {
	const runBtn = document.querySelector('#run');
	const codeInput = document.querySelector('#code');
	const sessionInput = document.querySelector('#session');
	const resultLbl = document.querySelector('#result');

	// address to Fluence contract in Ethereum blockchain. 
	// Interaction with blockchain created by MetaMask or with local/remote Ethereum node
	let contractAddress = "0xeFF91455de6D4CF57C141bD8bF819E5f873c1A01";

	// set ethUrl to `undefined` to use MetaMask instead of Ethereum node
	let ethUrl = "http://rinkeby.fluence.one:8545/"

	// application to interact with that stored in Fluence contract
	let appId = "381";

	// save fluence to global variable, so it can be accessed from Developer Console
	window.fluence = fluence;

	// create a session between client and backend application
	fluence.connect(contractAddress, appId, ethUrl).then((s) => {
		console.log("Session created");
		window.session = s;
		runBtn.disabled = false;
	});

	// window.session = fluence.directConnect("localhost", 30000, 1);
	// runBtn.disabled = false;

	// convert result to a string
	window.getResultString = function (result) {
		return result.result().then((r) => r.asString())
	};

	window.logResultAsString = function(result) {
		return getResultString(result).then((r) => console.log(r))
	}

	// set callback on button click
	runBtn.addEventListener("click", send)
	
	// send username as a transaction and display results in grettingLbl
	function send() {
		const req = JSON.stringify({
			action: "Run",
			code: codeInput.value.trim(),
			session: sessionInput.value.trim(),
		})
		let result = session.request(req);
		getResultString(result).then(function (str) {
			resultLbl.value = str;
		});
	}

};
