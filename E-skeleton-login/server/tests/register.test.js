const chai = require("chai")
const chaiHttp = require("chai-http")
const expect = chai.expect
chai.use(chaiHttp)

const app = require("../app.js")
const loginRequest = require("../requests/loginRequests")
const mongoose = require("mongoose")


describe("Register Tests", () => {

    before((done) => {
		mongoose.connect("mongodb://localhost:27017/mongologin", { useNewUrlParser: true, useFindAndModify: false }, (err) => { done() })
      });

	it("Register without sending parameters", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({})
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Email error: email field missing")
	})

    it("Register without sending password", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password error: password field missing")
	})

	it("Register with unvalid password no Uppercase", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott",
            password: "ihatetobyflenderson"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password error: There are no upercase letter")
	})

	it("Register with unvalid password no numbers", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott",
            password: "IHateTobyFlenderson"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password error: There are no numbers")
	})

	it("Register user without password confirmation", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott",
            password: "IHateTobyFlenderson4ever"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("Password confirmation error: passwordConfirmation field missing")
	})

	it("Register user", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott",
            password: "IHateTobyFlenderson4ever",
            passwordConfirmation: "IHateTobyFlenderson4ever"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("200")
    	expect(res.body.status).to.equal("Register Succesfull")
	})

	it("Register already registered user", async () => {
    	let res = await chai.request(app).post('/register/register_user').send({
            email: "michaelscott@dundermifflin.com",
            username: "Michael Scott",
            password: "IHateTobyFlenderson4ever",
            passwordConfirmation: "IHateTobyFlenderson4ever"
        })
    	expect(res.status).to.equal(200)
    	expect(res.body.code).to.equal("400")
    	expect(res.body.status).to.equal("User already exists")
	})


    after((done) => {
        chai.request(app).get("/deleteUsers").then(() => {
            mongoose.disconnect(done);
        })
    });
})