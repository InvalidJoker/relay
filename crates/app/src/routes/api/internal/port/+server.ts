import {z} from "zod";
import {error, json} from "@sveltejs/kit";
import type {RequestHandler} from "./$types";

// TODO: check if comes from relay
export const GET: RequestHandler = async ({ request }) => {
   // return all persistent ports which need to stay free and compress to ranges so we dont send as many ports to the client
    return json({
        ports: []
    });
}