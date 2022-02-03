const express = require("express")
const router = express.Router()

// Modules
const User = require("../mongoDB/userModel.js")
const ValidationManager = require("../tools/validation.js")


router.post("/user_status", async (req, res) => {
	let body = req.body

	// Verify request data
	let validation = ValidationManager.validateEmailData(body)
	if (validation.isError) {
		return res.status(200).send(validation.errorMessage)
	}

	// Decrypt and search user
	await User.findOne({ email: body.email })
		.then((result) => {
			result ? res.status(200).send({ code: "200", status: result.status }) : res.status(200).send({ code: "400", status: "User doesn't exist" });
		})
		.catch((err) => { console.log(err); res.status(200).send(err.errorMessage) })
});

module.exports = router;