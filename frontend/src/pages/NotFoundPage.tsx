import React from "react";
import { Container, Title, Text, Button, Stack } from "@mantine/core";
import { useNavigate } from "react-router-dom";

const NotFound: React.FC = () => {
  const navigate = useNavigate();

  return (
    <Container
      size="lg"
      style={{
        height: "100vh",
        display: "flex",
        justifyContent: "center",
      }}
    >
      <Stack align="center" justify="center" ta={"center"}>
        <Title order={1} size={80} w={900}>
          404
        </Title>
        <Text size="xl" w={700} color="dimmed">
          Oops! The page you're looking for doesn't exist.
        </Text>
        <Button
          size="lg"
          variant="gradient"
          gradient={{ from: "blue", to: "cyan" }}
          onClick={() => navigate("/")}
        >
          Go Home
        </Button>
      </Stack>
    </Container>
  );
};

export default NotFound;
