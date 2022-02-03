const chai = require("chai")
const chaiHttp = require("chai-http")
const expect = chai.expect
chai.use(chaiHttp)

const app = require("../app.js")
const loginRequest = require("../requests/loginRequests")
const mongoose = require("mongoose")


describe("Login Tests", () => {

    before((done) => {
		mongoose.connect("mongodb://localhost:27017/mongologin", { useNewUrlParser: true, useFindAndModify: false }, (err) => { done() })
      });

	it("Login without sending parameters", async () => {
    	let res = await chai.request(app).post('/login/log_user').send({})
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Email error: email field missing")
	})

	it("Login without sending password", async () => {
    	let res = await chai.request(app).post('/login/log_user').send({
            email: "michaelscott@dundermifflin.com"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password error: password field missing")
	})

	it("Login with unvalid password no Uppercase", async () => {
    	let res = await chai.request(app).post('/login/log_user').send({
            email: "michaelscott@dundermifflin.com",
            password: "ihatetobyflenderson"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password error: There are no upercase letter")
	})

	it("Login with unvalid password no numbers", async () => {
    	let res = await chai.request(app).post('/login/log_user').send({
            email: "michaelscott@dundermifflin.com",
            password: "IHateTobyFlenderson"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password error: There are no numbers")
	})

	it("Login without being registered", async () => {
    	let res = await chai.request(app).post('/login/log_user').send({
            email: "michaelscott@dundermifflin.com",
            password: "IHateTobyFlenderson4ever"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("User doesn't exist")
	})

	it("Login request", async () => {
		console.log("enter1")
        chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott",
            password: "IHateTobyFlenderson4ever",
            passwordConfirmation: "IHateTobyFlenderson4ever"
        }).end(async (err, res1) => {
			console.log("out1")


			console.log("enter2")
			let res = await chai.request(app).post('/login/log_user').send({
				email: "michaelscott@dundermifflin.com",
				password: "IHateTobyFlenderson4ever"
			})
			console.log("out2")
			expect(res.status).to.equal(200)
			expect(res.body.code).to.equal("200")
			expect(res.body.status).to.equal("Login Succesfull")
		})

	})

    it("Login with wrong password", async () => {
    	let res = await chai.request(app).post('/login/log_user').send({
            email: "michaelscott@dundermifflin.com",
            password: "IForgotMyPassword4"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Wrong Password")
	})

    after((done) => {
        chai.request(app).get("/deleteUsers").then(() => {
            mongoose.disconnect(done);
        })
    });
})