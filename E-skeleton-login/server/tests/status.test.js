const chai = require("chai");
const chaiHttp = require("chai-http");
const expect = chai.expect;
chai.use(chaiHttp);

const app = require("../app.js");
const loginRequest = require("../requests/loginRequests");
const mongoose = require("mongoose");
let token = new String();

describe("Status Tests", () => {
  before((done) => {
    mongoose.connect(
      "mongodb://localhost:27017/mongologin",
      { useNewUrlParser: true, useFindAndModify: false },
      async (err) => {
        // Register user
        await chai.request(app).post("/register/register_user").send({
          email: "michaelscott@dundermifflin.com",
          username: "Michael Scott",
          password: "IHateTobyFlenderson4ever",
          passwordConfirmation: "IHateTobyFlenderson4ever",
        });

        done();
      }
    );
  });

  it("Logged in Status", async () => {
    // Login user
    let loginRequest = await chai.request(app).post("/login/log_user").send({
      email: "michaelscott@dundermifflin.com",
      password: "IHateTobyFlenderson4ever",
    });
    token = loginRequest.body.token;

    let res = await await chai
      .request(app)
      .post("/status/user_status")
      .set({ authorization: token })
      .send({
        email: "michaelscott@dundermifflin.com",
        password: "IHateTobyFlenderson4ever",
      });

    expect(res.status).to.equal(200);
    expect(res.body.code).to.equal("200");
    expect(res.body.status).to.equal("logged in");
  });

  it("Logged out Status", async () => {
    // Log out
    await chai.request(app).post("/logout/logout_user").send({
      email: "michaelscott@dundermifflin.com",
      password: "IHateTobyFlenderson4ever",
    });

    await chai
      .request(app)
      .post("/logout/logout_user")
      .set({ authorization: token })
      .send({
        email: "michaelscott@dundermifflin.com",
        password: "IHateTobyFlenderson4ever",
      });

    let res = await chai
      .request(app)
      .post("/status/user_status")
      .set({ authorization: token })
      .send({
        email: "michaelscott@dundermifflin.com",
        password: "IHateTobyFlenderson4ever",
      });

    expect(res.status).to.equal(200);
    expect(res.body.code).to.equal("200");
    expect(res.body.status).to.equal("logged out");
  });

  after((done) => {
    chai
      .request(app)
      .get("/deleteUsers")
      .then(() => {
        mongoose.disconnect(done);
      });
  });
});
