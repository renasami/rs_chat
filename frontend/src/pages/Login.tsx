import { useState } from "react";
import { TextInput, Button, Stack, Text } from "@mantine/core";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";

const Login: React.FC = () => {
  const [username, setUserName] = useState("");
  const [password, setPassword] = useState("");
  const { loginMutation } = useAuth();
  const navigate = useNavigate();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    const response = await loginMutation.mutateAsync({ username, password });
    console.log(response);
    navigate("/");
  };

  return (
    <Stack>
      <TextInput
        label="ユーザー名"
        value={username}
        onChange={(e) => setUserName(e.currentTarget.value)}
      />
      <TextInput
        label="パスワード"
        type="password"
        value={password}
        onChange={(e) => setPassword(e.currentTarget.value)}
      />
      <Button onClick={handleLogin} fullWidth>
        ログイン
      </Button>
      <Text size="sm" ta="center">
        アカウントがないですか？ <Link to="/register">登録する</Link>
      </Text>
    </Stack>
  );
};

export default Login;
