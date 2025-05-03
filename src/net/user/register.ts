import {defaultFailure, post} from "../../net/base.js";

export function register(username: string, nickname: string, password: string, email: string, code: string,  success: (arg0: any) => void, failure = defaultFailure) {
    post('/user/register', {
        username,
        nickname,
        password,
        email,
        code
    }, {
        'Content-Type': 'application/x-www-form-urlencoded'
    }, (data: any) => {
        if (data.code === 0) {
            success(data);
        }else {
            failure(data.message);
        }

    }, (message) => {
        failure(message);
    }, (err : Error) => {
        failure(err.message);
    });
}