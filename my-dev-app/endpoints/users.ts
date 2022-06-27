import { ChiselRequest, responseFromJson } from "@chiselstrike/api"
import { User } from "../models/user"

export default async function chisel(req: ChiselRequest) {
    if (req.method == 'POST') {
        const payload = await req.json();
        const username = "USERNAME";
        //const email = payload["email"];
        const password = "PASSWORD";
        const salt = "SALT";
        const empty_field = "EMPTY";
        const created = await User.create({ username, password: password, passwordSalt: salt, uploadedKey: empty_field });
        return responseFromJson({id: created.id});
    } else if (req.method == 'GET') {
        // if we have a parameter, treat it as an id, otherwise get all
        const id = req.pathComponents()[0]
        if (id) {
           const user = await User.findOne({id})
           const status = user ? 200 : 404;
           return responseFromJson(user, status)
        } else {
           const users = await User.cursor().toArray();
           return responseFromJson(users);
        }
    } else {
        return new Response("Wrong method", { status: 405});
    }
}