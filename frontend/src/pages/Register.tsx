import { useState } from "react";
import { TextInput, Button, Stack, Text } from "@mantine/core";
import { Link } from "react-router-dom";

const Register: React.FC = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [username, setUsername] = useState("");

  const handleRegister = () => {
    console.log("Registering with:", { username, email, password });
  };

  return (
    <Stack>
      <TextInput
        label="ユーザー名"
        value={username}
        onChange={(e) => setUsername(e.currentTarget.value)}
      />
      <TextInput
        label="メールアドレス"
        value={email}
        onChange={(e) => setEmail(e.currentTarget.value)}
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
