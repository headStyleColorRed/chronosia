
/*  #  #  #  #  #  #  #  #  #  #  #		L O G I N  &&  R E G I S T E R I N G	 #  #  #  #  #  #  #  #  #  #  #  #  #  #  */

function validateLoginData(userData) {
	let validationResult = {
		isError: false,
		errorMessage: new String()
	}

	if (validateUsername(userData.username).length > 1) {
		validationResult.isError = true
		validationResult.errorMessage = validateUsername(userData.username)
		return validationResult
	}

	if (validatePassword(userData.password).length > 1) {
		validationResult.isError = true
		validationResult.errorMessage = validatePassword(userData.password)
		return validationResult
	}

	return validationResult
}


function validateRegisterData(userData) {
	let validationResult = {
		isError: false,
		errorMessage: new String()
	}

	if (validateUsername(userData.username).length > 1) {
		validationResult.isError = true
		validationResult.errorMessage = validateUsername(userData.username)
		return validationResult
	}

	if (validatePassword(userData.password).length > 1) {
		validationResult.isError = true
		validationResult.errorMessage = validatePassword(userData.password)
		return validationResult
	}

	if (userData.password != userData.passwordConfirmation) {
		validationResult.isError = true
		validationResult.errorMessage = "Passwords do not match"
		return validationResult
	}

	return validationResult
}

// Validators
function validateUsername(str) {
	var error = "";

	if (!str)
		error = "Email error: Email field missing"
	else if (str == "") {
		error = "Email error: Empty username";
	} else if ((str.length < 5) || (str.length > 40)) {
		console.log(str.length);
		error = "Email error: Email must have 5-40 characters";
	} else if (!str.includes("@")) {
		error = "Email error: Email is not valid, missing @";
	} else if (!str.includes(".com")) {
		error = "Email error: Email is not valid, missing .com";
	} else {
		error = "";
	}
	return error;
}

function validatePassword(str) {

	if (!str)
		return "Password error: Password field missing"

	// Validate lowercased letters
	var lowerCaseLetters = /[a-z]/g;
	if (!str.match(lowerCaseLetters)) {
		return "Password error: There are no lowercase letter"
	}

	// Validate capital letters
	var upperCaseLetters = /[A-Z]/g;
	if (!str.match(upperCaseLetters)) {
		return "Password error: There are no upercase letter"
	}

	// Validate numbers
	var numbers = /[0-9]/g;
	if (!str.match(numbers)) {
		return "Password error: There are no numbers"
	}

	// Validate length
	if (!str.length >= 8) {
		return "Password error: Password is not long enough"
	}

	return ""
}



module.exports = {
	validateLoginData,
	validateRegisterData,
}