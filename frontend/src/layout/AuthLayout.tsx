import { Outlet } from "react-router-dom";
import { Box, Center, Container, Paper, Title } from "@mantine/core";
import { IconBrandRust } from "@tabler/icons-react";

const AuthLayout: React.FC = () => {
  return (
    <Center style={{ height: "100vh" }}>
      <Container size="xs">
        <Paper shadow="md" p="xl" radius="md" withBorder>
          <Box display={"flex"}>
            <IconBrandRust size="30" />
            <Title order={2} ta="center" mb="md">
              Rust Chat App
            </Title>
          </Box>
          <Outlet />
        </Paper>
      </Container>
    </Center>
  );
};

export default AuthLayout;
