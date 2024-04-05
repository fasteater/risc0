import "@risc0/ui/styles/globals.css";
import "~/styles/styles.css";

import { cn } from "@risc0/ui/cn";
import { Analytics } from "@vercel/analytics/react";
import { JetBrains_Mono as FontMono } from "next/font/google";
import type { PropsWithChildren } from "react";
import { Providers } from "~/client/providers";

export const metadata = {
  title: {
    template: "%s | RISC Zero Reports",
    default: "RISC Zero Reports",
  },
  description: "Get to market fast with dramatically lower development costs on the first general purpose zkVM.",
  icons: [
    {
      rel: "icon",
      url: "/favicon.png",
    },
  ],
};

const fontMono = FontMono({
  subsets: ["latin"],
  variable: "--font-jetbrains-mono",
});

export default function RootLayout({ children }: PropsWithChildren) {
  return (
    <html lang="en" suppressHydrationWarning className="h-full">
      <body className={cn("flex min-h-full flex-col font-sans", fontMono.variable)}>
        <Providers>{children}</Providers>
        <Analytics />
      </body>
    </html>
  );
}