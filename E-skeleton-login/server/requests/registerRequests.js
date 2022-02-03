const express = require("express")
const router = express.Router()
const bcrypt = require('bcrypt')

const User = require("../mongoDB/userModel.js")
const ValidationManager = require("../tools/validation.js")

router.post("/register_user", async (req, res) => {
	// Parse request Data
	let body = req.body

	let validation = ValidationManager.validateRegisterData(body)
	if (validation.isError) {
		return res.status(200).send({ code: "400", status: validation.errorMessage })
	}

	// Encrypt and create user
	const hash = await bcrypt.hash(body.password, 10);
	const user = new User({email: body.email, password: hash});

	try {
		await user.save().catch((err) => { throw err })
	} catch (err) {
		return res.status(200).send({ code: "400", status: err.code == 11000 ? "User already exists" : err}) 
	}

	res.status(200).send({ code: "200", status: "Register Succesfull"})
});


module.exports = router;