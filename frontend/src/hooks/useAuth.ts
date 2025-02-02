import { useMutation, useQueryClient, useQuery } from "react-query";
import { api, setAuthToken } from "../api/api";

// ユーザー情報の型
export type User = {
  user_id: string;
  username: string;
};

// 認証フック
export const useAuth = () => {
  const queryClient = useQueryClient();

  // ログイン処理
  const loginMutation = useMutation(api.login, {
    onSuccess: (data) => {
      setAuthToken(data.access_token);
      queryClient.setQueryData("user", data);
    },
  });

  // ユーザー登録処理
  const registerMutation = useMutation(api.register, {
    onSuccess: (data) => {
      setAuthToken(data.access_token);
      queryClient.setQueryData("user", data);
    },
  });

  // ログアウト処理
  const logoutMutation = useMutation(api.logout, {
    onSuccess: () => {
      setAuthToken(null);
      queryClient.removeQueries("user");
    },
  });

  // ユーザー情報取得
  const userQuery = useQuery<User | null>("user", api.getUser, {
    retry: false,
    onError: () =>  setAuthToken(null),
  });

  return {
    user: userQuery.data === undefined || null ? null : userQuery.data,
    isLoading: userQuery.isLoading,
    loginMutation,
    registerMutation,
    logoutMutation,
  };
};
