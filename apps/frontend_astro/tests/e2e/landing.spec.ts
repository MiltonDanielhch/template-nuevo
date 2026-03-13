import { expect, test } from "@playwright/test";

function randomEmail() {
  const id = Math.random().toString(36).substring(2, 10);
  return `e2e+${id}@lab3026.com`;
}

test.describe("Landing page - lead flow", () => {
  test("should load landing and show main sections", async ({ page }) => {
    await page.goto("/");

    const heading = await page.locator("h1").innerText();
    expect(heading).toMatch(/Gestiona usuarios|Manage users/);

    await expect(page.locator("text=/Regístrate|Sign up|Sign in/")).toHaveCount(1);
  });

  test("should submit lead form successfully", async ({ page }) => {
    await page.goto("/");
    const email = randomEmail();

    await page.fill("input[name=email]", email);
    await page.click('button:has-text("¡Avísame")');

    await expect(page.locator("text=¡Gracias!")).toBeVisible({ timeout: 10000 });
  });

  test("should show error for invalid email", async ({ page }) => {
    await page.goto("/");

    await page.fill("input[name=email]", "invalid-email");
    await page.click('button:has-text("¡Avísame")');

    await expect(page.locator("text=Ingresa un correo válido")).toBeVisible({ timeout: 5000 });
  });

  test("should reject duplicate email", async ({ page }) => {
    await page.goto("/");
    const email = randomEmail();

    await page.fill("input[name=email]", email);
    await page.click('button:has-text("¡Avísame")');
    await expect(page.locator("text=¡Gracias!")).toBeVisible({ timeout: 10000 });

    await page.goto("/");
    await page.fill("input[name=email]", email);
    await page.click('button:has-text("¡Avísame")');

    await expect(page.locator(/ya está registrado|already registered/)).toBeVisible({
      timeout: 10000,
    });
  });

  test("should have honeypot field for spam protection", async ({ page }) => {
    await page.goto("/");

    const honeypot = page.locator('input[name="honeypot"]');
    await expect(honeypot).toBeHidden();

    const email = randomEmail();
    await page.fill("input[name=email]", email);
    await page.fill('input[name="honeypot"]', "spam-bot-fill");
    await page.click('button:has-text("¡Avísame")');

    await expect(page.locator("text=¡Gracias!")).not.toBeVisible({ timeout: 5000 });
  });

  test("should be rate limited after multiple submissions", async ({ page }) => {
    await page.goto("/");

    for (let i = 0; i < 5; i++) {
      const uniqueEmail = `e2e+${i}+${Date.now()}@lab3026.com`;
      await page.goto("/");
      await page.fill("input[name=email]", uniqueEmail);
      await page.click('button:has-text("¡Avísame")');
    }

    const rateLimitedEmail = `e2e+rate+${Date.now()}@lab3026.com`;
    await page.fill("input[name=email]", rateLimitedEmail);
    await page.click('button:has-text("¡Avísame")');

    await expect(page.locator(/Demasiadas solicitudes|Too many requests|rate limit/i)).toBeVisible({
      timeout: 10000,
    });
  });
});
