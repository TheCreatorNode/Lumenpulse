import { Module } from '@nestjs/common';
import { CacheModule as NestCacheModule } from '@nestjs/cache-manager';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { CacheService } from './cache.service';

@Module({
  imports: [
    NestCacheModule.registerAsync({
      isGlobal: true,
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => {
        const ttl = configService.get<number>('CACHE_TTL_MS', 300_000);
        return {
          ttl,
        };
      },
      inject: [ConfigService],
    }),
  ],
  providers: [CacheService],
  exports: [CacheService],
})
export class AppCacheModule {}
