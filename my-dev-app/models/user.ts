import { AuthUser, ChiselEntity, labels, unique } from "@chiselstrike/api"

export class User extends ChiselEntity {
    // @unique username: string;
    username: string;
    // @unique declare email: string;
    @labels("hide") password: string;
    @labels("hide") passwordSalt: string;
    @labels("empty") uploadedKey: string;
}