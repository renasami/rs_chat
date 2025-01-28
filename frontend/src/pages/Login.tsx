import { useState } from "react";
import { TextInput, Button, Stack, Text } from "@mantine/core";
import { Link } from "react-router-dom";

const Login: React.FC = () => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  const handleLogin = () => {
    console.log("Logging in with:", { email, password });
  };

  return (
    <Stack>
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
