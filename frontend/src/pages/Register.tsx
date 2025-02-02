import { useState } from "react";
import { TextInput, Button, Stack, Text } from "@mantine/core";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../hooks/useAuth";

const Register: React.FC = () => {
  const [password, setPassword] = useState("");
  const [username, setUsername] = useState("");

  const { registerMutation } = useAuth();
  const navigate = useNavigate();

  const handleRegister = async (e: React.FormEvent) => {
    e.preventDefault();
    const response = await registerMutation.mutateAsync({ username, password });
    console.log(response);
    navigate("/");
  };

  return (
    <Stack>
      <TextInput
        label="ユーザー名"
        value={username}
        onChange={(e) => setUsername(e.currentTarget.value)}
      />
      <TextInput
        label="パスワード"
        type="password"
        value={password}
        onChange={(e) => setPassword(e.currentTarget.value)}
      />
      <Button onClick={handleRegister} fullWidth>
        登録
      </Button>
      <Text size="sm" ta="center">
        すでにアカウントをお持ちですか？ <Link to="/login">ログインする</Link>
      </Text>
    </Stack>
  );
};

export default Register;
