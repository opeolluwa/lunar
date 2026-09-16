import { useAuthStore } from "@shared/stores/auth";
import type { FetchError } from "ofetch";
import { $fetch } from "ofetch";
import type {
  AcceptInvitationRequest,
  AcceptInvitationResponse,
  CreateUserRequest,
  CreateUserResponse,
  ForgottenPasswordRequest,
  ForgottenPasswordResponse,
  LoginRequest,
  LoginResponse,
  ResendOtpRequest,
  SetNewPasswordRequest,
  SetNewPasswordResponse,
  VerifyAccountRequest,
  VerifyAccountResponse,
} from "lunar";

interface ApiErrorBody {
  message?: string;
}

export function useAuthApi() {
  const config = useRuntimeConfig();
  const authStore = useAuthStore();

  const baseUrl = computed(() =>
    String(config.public.apiBaseUrl)
      .replace(/\/+$/, "")
      .replace(/\/orchard$/, ""),
  );

  async function post<T>(
    path: string,
    body?: unknown,
    requestOptions: { token?: string } = {},
  ): Promise<T> {
    const headers: Record<string, string> = {};
    const token = requestOptions.token ?? authStore.accessToken;
    if (token) headers.Authorization = `Bearer ${token}`;

    try {
      return await $fetch<T>(path, {
        baseURL: baseUrl.value,
        method: "POST",
        body,
        headers,
      });
    } catch (error) {
      const err = error as FetchError;
      const message = (err.data as ApiErrorBody | undefined)?.message;
      if (message) throw new Error(message, { cause: error });
      throw new Error("Something went wrong. Please try again.", {
        cause: error,
      });
    }
  }

  return {
    baseUrl,
    post,
    signup: (req: CreateUserRequest) =>
      post<CreateUserResponse>("/auth/signup", req),
    login: (req: LoginRequest) => post<LoginResponse>("/auth/login", req),
    forgottenPassword: (req: ForgottenPasswordRequest) =>
      post<ForgottenPasswordResponse>("/auth/forgotten-password", req),
    verifyAccount: (req: VerifyAccountRequest, token: string) =>
      post<VerifyAccountResponse>("/auth/verify-account", req, { token }),
    verifyResetOtp: (req: VerifyAccountRequest, token: string) =>
      post<VerifyAccountResponse>("/auth/verify", req, { token }),
    setNewPassword: (req: SetNewPasswordRequest, token: string) =>
      post<SetNewPasswordResponse>("/auth/reset-password", req, { token }),
    acceptInvitation: (req: AcceptInvitationRequest) =>
      post<AcceptInvitationResponse>("/invitations/accept", req),
    resendOtp: (req: ResendOtpRequest, token: string) =>
      post<ForgottenPasswordResponse>("/auth/resend-otp", req, { token }),
  };
}
