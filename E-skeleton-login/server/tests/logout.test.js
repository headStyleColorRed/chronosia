const chai = require("chai");
const chaiHttp = require("chai-http");
const expect = chai.expect;
chai.use(chaiHttp);

const app = require("../app.js");
const mongoose = require("mongoose");
let token = new String();

describe("Logout Tests", () => {
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
        // Login user
        let loginRequest = await chai
          .request(app)
          .post("/login/log_user")
          .send({
            email: "michaelscott@dundermifflin.com",
            password: "IHateTobyFlenderson4ever",
          });
        token = loginRequest.body.token;

        done();
      }
    );
  });

  it("Logout with wrong email", async () => {
    let res = await chai
      .request(app)
      .post("/logout/logout_user")
      .set({ authorization: token })
      .send({
        email: "michaelcott@dundermifflin.com",
        password: "IHateTobyFlenderson4ever",
      });

    expect(res.status).to.equal(200);
    expect(res.body.code).to.equal("400");
    expect(res.body.status).to.equal("Logout Error");
  });

  it("Logout with wrong password", async () => {
    let res = await chai
      .request(app)
      .post("/logout/logout_user")
      .set({ authorization: token })
      .send({
        email: "michaelscott@dundermifflin.com",
        password: "IHateTobyFlenderson4eve",
      });

    expect(res.status).to.equal(200);
    expect(res.body.code).to.equal("400");
    expect(res.body.status).to.equal("Logout Error");
  });

  it("Logout request", async () => {
    // Loggout user
    let res = await chai
      .request(app)
      .post("/logout/logout_user")
      .set({ authorization: token })
      .send({
        email: "michaelscott@dundermifflin.com",
        password: "IHateTobyFlenderson4ever",
      });

    expect(res.status).to.equal(200);
    expect(res.body.code).to.equal("200");
    expect(res.body.status).to.equal("Logout Succesfull");
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
